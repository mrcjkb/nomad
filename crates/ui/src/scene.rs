use alloc::borrow::Cow;
use alloc::vec::Drain;
use core::cmp::Ordering;
use core::mem;

use compact_str::CompactString;

use crate::{Bound, Cells, Metric, SceneFragment, Surface};

/// TODO: docs
#[derive(Debug, Default)]
pub(crate) struct Scene {
    /// TODO: docs.
    lines: Vec<SceneLine>,

    /// TODO: docs.
    diff: DiffTracker,
}

impl Scene {
    #[inline]
    fn apply(&mut self, resize_op: ResizeOp) {
        resize_op.apply_to(self);
    }

    /// Turns the entire `Scene` into a `SceneFragment` which can be used in
    /// the [`paint`](crate::Render::paint) method of a
    /// [`Render`](crate::Render) implementation.
    #[inline]
    pub(crate) fn as_fragment(&mut self) -> SceneFragment<'_> {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn diff(&mut self) -> SceneDiff<'_> {
        let resize = mem::take(&mut self.diff.resize);
        let paint = self.diff.paint.drain(..);
        SceneDiff { lines: &self.lines, resize, paint }
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn height(&self) -> Cells {
        (self.lines.len() as u32).into()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn resize(&mut self, new_size: Bound<Cells>) {
        let op = ResizeOp::new(self.size(), new_size);
        self.apply(op);
        self.diff.resize = op;
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn size(&self) -> Bound<Cells> {
        Bound::new(self.height(), self.width())
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn width(&self) -> Cells {
        self.lines.first().map(SceneLine::width).unwrap_or_default()
    }
}

/// TODO: docs
#[derive(Debug)]
struct SceneLine {
    runs: Vec<SceneRun>,
}

impl SceneLine {
    /// TODO: docs.
    #[inline]
    fn extend(&mut self, to_width: Cells) {
        if to_width > self.width() {
            let cells = to_width - self.width();
            self.runs.push(SceneRun::new_empty(cells));
        }
    }

    /// Creates a new empty `SceneLine` with the given width.
    #[inline]
    fn new_empty(width: Cells) -> Self {
        Self { runs: vec![SceneRun::new_empty(width)] }
    }

    /// TODO: docs.
    #[inline]
    fn run_at_offset(&self, offset: Cells, bias: Bias) -> (usize, Cells) {
        let mut run_offset = Cells::zero();
        let mut runs = self.runs.iter().enumerate();

        loop {
            let Some((mut run_idx, run)) = runs.next() else {
                panic!("offset out of bounds");
            };

            match (run_offset + run.width()).cmp(&offset) {
                Ordering::Less => {
                    run_offset += run.width();
                },

                Ordering::Equal => {
                    if bias == Bias::Right {
                        if let Some((next_idx, _)) = runs.next() {
                            run_idx = next_idx;
                            run_offset += run.width();
                        }
                    }

                    return (run_idx, run_offset);
                },

                Ordering::Greater => {
                    return (run_idx, run_offset);
                },
            }
        }
    }

    /// TODO: docs.
    #[inline]
    fn truncate(&mut self, to_width: Cells) {
        let (run_idx, run_offset) = self.run_at_offset(to_width, Bias::Right);

        if run_offset < to_width {
            self.runs[run_idx].truncate(to_width - run_offset);
            self.runs.truncate(run_idx + 1);
        } else {
            self.runs.truncate(run_idx);
        }
    }

    /// TODO: docs.
    #[inline]
    fn width(&self) -> Cells {
        // FIXME: this is O(n). We could do it in O(1) by either memoizing it
        // or by storing the runs in a Btree.
        self.runs.iter().map(SceneRun::width).sum()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bias {
    Left,
    Right,
}

/// TODO: docs
#[derive(Debug)]
enum SceneRun {
    /// TODO: docs.
    Empty { width: Cells },

    /// TODO: docs.
    Text { text: CompactString },
}

impl SceneRun {
    /// Creates a new empty `SceneRun` with the given width.
    #[inline]
    fn new_empty(width: Cells) -> Self {
        Self::Empty { width }
    }

    /// Returns the text of the `SceneRun`.
    #[inline]
    fn text(&self) -> Cow<str> {
        /// The sole purpose of this constant is to avoid allocating when the
        /// text is empty.
        const SPACES: &str = r#"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "#;

        match self {
            Self::Empty { width } => {
                let len = u32::from(*width) as usize;
                if len > SPACES.len() {
                    Cow::Owned(" ".repeat(len))
                } else {
                    Cow::Borrowed(&SPACES[..len])
                }
            },

            Self::Text { text } => Cow::Borrowed(text.as_str()),
        }
    }

    /// TODO: docs.
    #[inline]
    fn truncate(&mut self, to_width: Cells) {
        match self {
            Self::Empty { width } => *width = to_width,

            Self::Text { text } => {
                todo!("convert cells to byte_offset");
            },
        }
    }

    /// Returns the width of the `SceneRun`.
    ///
    /// This is equal to the number of terminal cells used to render the run's
    /// [`text`](Self::text).
    #[inline]
    fn width(&self) -> Cells {
        match self {
            Self::Empty { width } => *width,
            Self::Text { text } => Cells::measure(text.as_str()),
        }
    }
}

/// TODO: docs
#[derive(Debug, Default)]
struct DiffTracker {
    /// TODO: docs.
    resize: ResizeOp,

    /// TODO: docs
    paint: Vec<PaintOp>,
}

/// A `ResizeOp` is a collection of operations that resize a `Scene`.
#[derive(Debug, Copy, Clone, Default)]
struct ResizeOp {
    shrink: ShrinkOp,
    expand: ExpandOp,
}

impl ResizeOp {
    /// Applies the resize operations to a `Scene`.
    ///
    /// The [`size`](Scene::size) of the given scene is guaranteed to return
    /// `new_size` after this method is called, where `new_size` is the new
    /// size passed to [`ResizeOp::new`].
    #[inline]
    fn apply_to(self, scene: &mut Scene) {
        self.shrink.apply_to(scene);
        self.expand.apply_to(scene);
    }

    #[inline]
    fn new(old_size: Bound<Cells>, new_size: Bound<Cells>) -> Self {
        let shrink = ShrinkOp::new(old_size, new_size);
        let expand = ExpandOp::new(old_size, new_size);
        Self { shrink, expand }
    }
}

/// A `ShrinkOp` shrinks a [`Scene`] by deleting lines and/or truncating lines.
#[derive(Debug, Copy, Clone, Default)]
struct ShrinkOp {
    delete_lines: Option<DeleteLinesOp>,
    truncate_lines: Option<TruncateLinesOp>,
}

impl ShrinkOp {
    #[inline]
    fn apply_to(self, scene: &mut Scene) {
        if let Some(delete_lines) = self.delete_lines {
            delete_lines.apply_to(scene);
        }

        if let Some(truncate_lines) = self.truncate_lines {
            truncate_lines.apply_to(scene);
        }
    }

    #[inline]
    fn new(old_size: Bound<Cells>, new_size: Bound<Cells>) -> Self {
        let delete_lines = if new_size.height() < old_size.height() {
            Some(DeleteLinesOp((old_size.height() - new_size.height()).into()))
        } else {
            None
        };

        let truncate_lines = if new_size.width() < old_size.width() {
            Some(TruncateLinesOp((old_size.width() - new_size.width()).into()))
        } else {
            None
        };

        Self { delete_lines, truncate_lines }
    }
}

/// A `DeleteLinesOp(n)` shrinks a [`Scene`] vertically by keeping its first
/// `n` lines and deleting the rest.
///
/// For example, a `DeleteLinesOp(1)` would transform the following scene:
///
/// ```txt
/// ┌──────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒3x14▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └──────────────┘
/// ```
///
/// into:
///
/// ```txt
/// ┌──────────────┐
/// │▒▒▒▒▒1x14▒▒▒▒▒│
/// └──────────────┘
/// ```
///
/// A `DeleteLinesOp(0)` deletes all the lines of a `Scene`.
#[derive(Debug, Clone, Copy)]
struct DeleteLinesOp(u32);

impl DeleteLinesOp {
    #[inline]
    fn apply_to(self, scene: &mut Scene) {
        scene.lines.truncate(self.0 as usize);
    }
}

/// A `TruncateLinesOp(n)` shrinks a [`Scene`] horizontally by keeping the
/// first `n` cells of every line and deleting the rest.
///
/// For example, a `TruncateLinesOp(10)` would transform the following scene:
///
/// ```txt
/// ┌──────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒3x14▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └──────────────┘
/// ```
///
/// into:
///
/// ```txt
/// ┌──────────┐
/// │▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒3x10▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒│
/// └──────────┘
/// ```
///
/// A `TruncateLinesOp(0)` deletes all the cells of a `Scene`.
#[derive(Debug, Clone, Copy)]
struct TruncateLinesOp(u32);

impl TruncateLinesOp {
    #[inline]
    fn apply_to(self, scene: &mut Scene) {
        let cells = Cells::from(self.0);
        scene.lines.iter_mut().for_each(|line| line.truncate(cells));
    }
}

/// An `ExpandOp` expands a `Scene` by inserting lines and/or extending lines.
#[derive(Debug, Clone, Copy, Default)]
struct ExpandOp {
    extend_lines: Option<ExtendLinesOp>,
    insert_lines: Option<InsertLinesOp>,
}

impl ExpandOp {
    #[inline]
    fn apply_to(self, _scene: &mut Scene) {
        if let Some(extend_lines) = self.extend_lines {
            extend_lines.apply_to(_scene);
        }

        if let Some(insert_lines) = self.insert_lines {
            insert_lines.apply_to(_scene);
        }
    }

    #[inline]
    fn new(old_size: Bound<Cells>, new_size: Bound<Cells>) -> Self {
        let extend_lines = if new_size.width() > old_size.width() {
            Some(ExtendLinesOp((new_size.width() - old_size.width()).into()))
        } else {
            None
        };

        let insert_lines = if new_size.height() > old_size.height() {
            Some(InsertLinesOp((new_size.height() - old_size.height()).into()))
        } else {
            None
        };

        Self { extend_lines, insert_lines }
    }
}

/// An `InsertLinesOp(n)` expands a [`Scene`] vertically by appending lines
/// until its height reaches `n` cells.
///
/// For example, an `InsertLinesOp(5)` would transform the following scene:
///
/// ```txt
/// ┌──────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒3x14▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └──────────────┘
/// ```
///
/// into:
///
/// ```txt
/// ┌──────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒5x14▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └──────────────┘
/// ```
#[derive(Debug, Clone, Copy)]
struct InsertLinesOp(u32);

impl InsertLinesOp {
    #[inline]
    fn apply_to(self, scene: &mut Scene) {
        let len = self.0 as usize;
        let width = scene.width();
        scene.lines.resize_with(len, || SceneLine::new_empty(width));
    }
}

/// An `ExtendLinesOp(n)` expands a [`Scene`] horizontally by extending every
/// line until its width reaches `n` cells.
///
/// For example, an `ExtendLinesOp(18)` would transform the following scene:
///
/// ```txt
/// ┌──────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒3x14▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └──────────────┘
/// ```
///
/// into:
///
/// ```txt
/// ┌──────────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒▒▒3x18▒▒▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └──────────────────┘
/// ```
#[derive(Debug, Clone, Copy)]
struct ExtendLinesOp(u32);

impl ExtendLinesOp {
    #[inline]
    fn apply_to(self, scene: &mut Scene) {
        let cells = Cells::from(self.0);
        scene.lines.iter_mut().for_each(|line| line.extend(cells));
    }
}

/// TODO: docs
#[derive(Debug)]
struct PaintOp {}

/// TODO: docs
pub(crate) struct SceneDiff<'a> {
    lines: &'a [SceneLine],
    resize: ResizeOp,
    paint: Drain<'a, PaintOp>,
}

impl<'a> SceneDiff<'a> {
    /// TODO: docs.
    #[inline]
    fn hl_hunks(&self) -> HlHunks<'_> {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    fn text_hunks(&self) -> TextHunks<'_> {
        todo!();
    }
}

impl<'a> SceneDiff<'a> {
    /// TODO: docs
    #[inline]
    pub(crate) fn apply_to(self, surface: &mut Surface) {
        for hunk in self.text_hunks() {
            hunk.apply_to(surface);
        }

        for hunk in self.hl_hunks() {
            hunk.apply_to(surface);
        }
    }
}

/// TODO: docs.
struct HlHunks<'a> {
    _marker: &'a (),
}

impl Iterator for HlHunks<'_> {
    type Item = HlHunk;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

/// TODO: docs.
#[derive(Debug)]
struct HlHunk {}

/// TODO: docs.
impl HlHunk {
    /// TODO: docs
    #[inline]
    fn apply_to(self, _surface: &mut Surface) {
        todo!();
    }
}

/// TODO: docs.
struct TextHunks<'a> {
    _marker: &'a (),
}

impl<'a> Iterator for TextHunks<'a> {
    type Item = TextHunk<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

/// TODO: docs.
#[derive(Debug)]
struct TextHunk<'a> {
    _marker: &'a (),
}

/// TODO: docs.
impl<'a> TextHunk<'a> {
    /// TODO: docs
    #[inline]
    fn apply_to(self, _surface: &mut Surface) {
        todo!();
    }
}
