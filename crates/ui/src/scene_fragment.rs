use alloc::rc::Rc;
use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::Range;
use core::panic::Location;
use core::ptr::NonNull;

use crate::scene::{SceneLineBorrow, SceneRunBorrow};
use crate::{Bound, Cells, HighlightGroup, Metric, Point, Scene};

/// TODO: docs.
pub struct SceneFragment<'scene> {
    /// TODO: docs.
    scene_ptr: NonNull<Scene>,

    /// The origin of the fragment.
    ///
    /// The area of the scene covered by this fragment is the rectangle given
    /// by [`Self::size`] with its top-left corner at this point.
    origin: Point<Cells>,

    /// The size of the fragment.
    size: Bound<Cells>,

    /// Whether the fragment is currently borrowed. This is needed to make
    /// accessing the scene via a raw pointer safe.
    borrow: Rc<Cell<Borrow>>,

    _lifetime: PhantomData<&'scene mut Scene>,
}

#[derive(Debug, Copy, Clone)]
enum Borrow {
    Borrowed(&'static Location<'static>),
    NotBorrowed,
}

impl<'scene> SceneFragment<'scene> {
    /// TODO: docs
    #[inline]
    pub fn cutout<C: Cutout>(self, cutout: C) -> (Self, C::Cutout<'scene>) {
        cutout.cutout(self)
    }

    /// TODO: docs
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }

    /// TODO: docs
    #[inline]
    pub fn height(&self) -> Cells {
        self.size.height()
    }

    /// TODO: docs
    #[track_caller]
    #[inline]
    pub fn lines(&mut self) -> FragmentLines<'_> {
        FragmentLines::new(self)
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new(scene: &'scene mut Scene) -> Self {
        Self {
            size: scene.size(),
            scene_ptr: NonNull::from(scene),
            origin: Point::origin(),
            borrow: Rc::new(Cell::new(Borrow::NotBorrowed)),
            _lifetime: PhantomData,
        }
    }

    /// TODO: docs
    #[inline]
    pub fn split_x(mut self, split_at: Cells) -> (Self, Self) {
        let mut bottom_origin = self.origin;
        *bottom_origin.y_mut() += split_at;

        let mut bottom_size = self.size;
        *bottom_size.height_mut() -= split_at;

        *self.size.height_mut() = split_at;

        let bottom_fragment = Self {
            scene_ptr: self.scene_ptr,
            origin: bottom_origin,
            size: bottom_size,
            borrow: self.borrow.clone(),
            _lifetime: PhantomData,
        };

        (self, bottom_fragment)
    }

    /// TODO: docs
    #[inline]
    pub fn split_y(mut self, split_at: Cells) -> (Self, Self) {
        let mut right_origin = self.origin;
        *right_origin.x_mut() += split_at;

        let mut right_size = self.size;
        *right_size.width_mut() -= split_at;

        *self.size.width_mut() = split_at;

        let right_fragment = Self {
            scene_ptr: self.scene_ptr,
            origin: right_origin,
            size: right_size,
            borrow: self.borrow.clone(),
            _lifetime: PhantomData,
        };

        (self, right_fragment)
    }

    /// TODO: docs
    #[inline]
    pub fn width(&self) -> Cells {
        self.size.width()
    }
}

/// TODO: docs.
pub struct FragmentLines<'fragment> {
    scene_ptr: NonNull<Scene>,
    borrow: Rc<Cell<Borrow>>,

    /// The index range of the scene lines that this iterator will yield.
    line_idxs: Range<usize>,

    /// The horizontal cell offset of the fragment within the scene.
    cell_offset: Cells,

    /// The width of the [`SceneFragment`] this iterator was created from.
    fragment_width: Cells,

    lifetime: PhantomData<&'fragment Scene>,
}

impl<'fragment> FragmentLines<'fragment> {
    #[track_caller]
    #[inline]
    fn new(fragment: &'fragment mut SceneFragment<'_>) -> Self {
        if let Borrow::Borrowed(at) = fragment.borrow.get() {
            panic!("fragment already borrowed at {at}")
        }

        fragment.borrow.set(Borrow::Borrowed(Location::caller()));

        let line_idxs = {
            let start = fragment.origin.y().as_usize();
            let height = fragment.size.height().as_usize();
            start..start + height
        };

        Self {
            scene_ptr: fragment.scene_ptr,
            line_idxs,
            borrow: fragment.borrow.clone(),
            cell_offset: fragment.origin.x(),
            fragment_width: fragment.size.width(),
            lifetime: PhantomData,
        }
    }
}

impl<'fragment> Iterator for FragmentLines<'fragment> {
    type Item = FragmentLine<'fragment>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let line_idx = self.line_idxs.next()?;

        // SAFETY: TODO.
        let scene = unsafe { self.scene_ptr.as_mut() };

        let (_, line) = scene.line_mut(line_idx).split(self.cell_offset);
        let (line, _) = line.split(self.fragment_width);

        Some(FragmentLine {
            inner: line,
            offset: Cells::zero(),
            fragment_width: self.fragment_width,
        })
    }
}

impl Drop for FragmentLines<'_> {
    #[inline]
    fn drop(&mut self) {
        self.borrow.set(Borrow::NotBorrowed);
    }
}

/// TODO: docs
pub struct FragmentLine<'fragment> {
    inner: SceneLineBorrow<'fragment>,
    offset: Cells,
    fragment_width: Cells,
}

impl<'fragment> FragmentLine<'fragment> {
    /// TODO: docs
    #[inline]
    pub fn into_run(self) -> FragmentRun<'fragment> {
        FragmentRun::new(self.inner.into_run())
    }

    /// TODO: docs
    #[inline]
    pub fn split_run(&mut self, split_at: Cells) -> Option<FragmentRun<'_>> {
        assert!(split_at <= self.width());

        if split_at == self.width() {
            None
        } else {
            self.inner.split_run(split_at).map(FragmentRun::new)
        }
    }

    /// TODO: docs
    #[inline]
    pub fn width(&self) -> Cells {
        self.fragment_width - self.offset
    }
}

/// TODO: docs
pub struct FragmentRun<'scene> {
    /// TODO: docs
    inner: SceneRunBorrow<'scene>,
}

impl<'scene> FragmentRun<'scene> {
    #[inline]
    fn new(inner: SceneRunBorrow<'scene>) -> Self {
        Self { inner }
    }

    /// TODO: docs
    #[inline]
    pub fn set_highlight(&mut self, hl_group: &HighlightGroup) {
        self.inner.set_hl_group(hl_group);
    }

    /// TODO: docs
    #[inline]
    pub fn set_text(&mut self, text: &str) {
        self.inner.set_text(text);
    }

    /// TODO: docs
    #[inline]
    pub fn width(&self) -> Cells {
        self.inner.width()
    }
}

/// TODO: docs.
pub trait Cutout {
    /// TODO: docs.
    type Cutout<'a>;

    /// TODO: docs.
    fn cutout(
        self,
        fragment: SceneFragment,
    ) -> (SceneFragment, Self::Cutout<'_>);
}
