use core::mem;
use core::ops::RangeBounds;
use core::str::FromStr;

use api::opts::{BufAttachOpts, OptionOpts};
use nvim_oxi::api::{self, Buffer};
use nvimx_common::{
    Apply,
    ByteLen,
    ByteOffset,
    LineIdx,
    Metric,
    Point2,
    Replacement,
    Shared,
};

/// TODO: docs.
pub struct TextBuffer {
    /// TODO: docs.
    attach_status: AttachStatus,

    /// TODO: docs.
    inner: BufferInner,
}

impl TextBuffer {
    /// Attaches to the buffer if not already attached, and returns a mutable
    /// reference to the [`AttachState`].
    #[inline]
    fn attach(&mut self) -> &mut Shared<AttachState> {
        if let AttachStatus::NotAttached = self.attach_status {
            let state = Shared::<AttachState>::default();

            let on_edit = {
                let state = state.clone();
                move |args| {
                    let replacement = Replacement::from(args);
                    state.with_mut(|state| state.on_edit(replacement));
                }
            };

            self.inner.attach(on_edit);
            self.attach_status = AttachStatus::Attached(state);
        }

        let AttachStatus::Attached(state) = &mut self.attach_status else {
            unreachable!("just attached if wasn't already")
        };

        state
    }

    /// TODO: docs.
    #[inline]
    pub fn current() -> Result<Self, NotTextBufferError> {
        let buffer = Buffer::current();

        match buffer.buftype() {
            Buftype::Text => Ok(Self::new(buffer)),
            Buftype::Help => Err(NotTextBufferError::Help),
            Buftype::Quickfix => Err(NotTextBufferError::Quickfix),
            Buftype::Terminal => Err(NotTextBufferError::Terminal),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn edit<E>(&mut self, edit: E) -> <Self as Apply<E>>::Diff
    where
        Self: Apply<E>,
    {
        if let AttachStatus::Attached(state) = &self.attach_status {
            state.with_mut(|state| state.edit_side = EditSide::Ours);
        }

        self.apply(edit)
    }

    /// Creates a new text buffer from the given [`Buffer`].
    ///
    /// # Panics
    ///
    /// Panics if the buffer's type is not [`Buftype::Text`].
    #[inline]
    fn new(inner: Buffer) -> Self {
        debug_assert!(inner.buftype().is_text());
        Self {
            attach_status: AttachStatus::NotAttached,
            inner: BufferInner::new(inner),
        }
    }
}

struct BufferInner(api::Buffer);

impl BufferInner {
    #[inline]
    fn attach<F>(&self, mut on_bytes: F)
    where
        F: FnMut(OnBytesArgs) + 'static,
    {
        let opts = BufAttachOpts::builder()
            .on_bytes(move |args: api::opts::OnBytesArgs| {
                on_bytes(args.into());
                Ok(false)
            })
            .build();

        self.0.attach(false, &opts).expect("todo");
    }

    #[inline]
    fn get<R>(&self, _range: R) -> Result<String, api::Error>
    where
        R: RangeBounds<Point2<LineIdx, ByteOffset>>,
    {
        todo!();
    }

    #[inline]
    fn new(inner: api::Buffer) -> Self {
        Self(inner)
    }
}

enum AttachStatus {
    Attached(Shared<AttachState>),
    NotAttached,
}

#[derive(Default)]
struct AttachState {
    /// Whether the edit was performed by calling [`Buffer::edit`].
    edit_side: EditSide,

    /// Callbacks registered to be called when the buffer is edited.
    on_edit_callbacks: Vec<Box<dyn FnMut(&Replacement<ByteOffset>, EditSide)>>,
}

impl AttachState {
    #[inline]
    fn on_edit(&mut self, replacement: Replacement<ByteOffset>) {
        let side = mem::take(&mut self.edit_side);

        for callback in &mut self.on_edit_callbacks {
            callback(&replacement, side);
        }
    }
}

/// TODO: docs.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum EditSide {
    /// TODO: docs.
    Ours,

    /// TODO: docs.
    #[default]
    Theirs,
}

struct OnBytesArgs {
    buffer: BufferInner,

    /// The line idx of the line containing the start of the replacing text.
    start_row: LineIdx,

    /// The byte offset in [`Self::start_row`] of the start of the replacing
    /// text.
    start_col: ByteOffset,

    /// The number of line indixes to add to `start_row` to get the line idx of
    /// the line containing the end of the replacing text.
    new_end_row: LineIdx,

    /// If [`new_end_row`](Self::new_end_row) is 0, this is the number of bytes
    /// to add to [`Self::start_col`] to get the byte offset of the end of the
    /// replacing text.
    ///
    /// If [`new_end_row`](Self::new_end_row) is not 0, this is the byte offset
    /// of the end of the replacing text.
    new_end_col: ByteLen,

    /// The byte offset in the buffer of the start of the replaced range.
    start_offset: ByteOffset,

    /// The byte length of the replaced range.
    old_end_len: ByteLen,
}

impl From<api::opts::OnBytesArgs> for OnBytesArgs {
    #[inline]
    fn from(
        (
            _bytes,
            buf,
            _changedtick,
            start_row,
            start_col,
            start_offset,
            _old_end_row,
            _old_end_col,
            old_end_len,
            new_end_row,
            new_end_col,
            _new_end_len,
        ): api::opts::OnBytesArgs,
    ) -> Self {
        Self {
            buffer: BufferInner(buf),
            start_row: start_row.into(),
            start_col: start_col.into(),
            new_end_row: new_end_row.into(),
            new_end_col: new_end_col.into(),
            start_offset: start_offset.into(),
            old_end_len: old_end_len.into(),
        }
    }
}

impl From<OnBytesArgs> for Replacement<ByteOffset> {
    #[inline]
    fn from(args: OnBytesArgs) -> Self {
        let start = Point2::new(args.start_row, args.start_col);

        let end = Point2::new(
            args.start_row + args.new_end_row,
            (args.start_col * (args.new_end_row.is_zero() as usize)
                + args.new_end_col)
                .into(),
        );

        let replacing_text = (start == end)
            .then(|| args.buffer.get(start..end).expect("always valid"));

        let replaced_range =
            args.start_offset..(args.start_offset + args.old_end_len);

        Replacement::new(
            replaced_range,
            replacing_text.as_deref().unwrap_or_default(),
        )
    }
}

trait BufferExt {
    fn buftype(&self) -> Buftype;
}

impl BufferExt for Buffer {
    #[inline]
    fn buftype(&self) -> Buftype {
        let opts = OptionOpts::builder().buffer(self.clone()).build();

        api::get_option_value::<String>("buftype", &opts)
            .expect("always set")
            .parse()
            .unwrap_or_else(|other| panic!("unknown buftype: {other}"))
    }
}

enum Buftype {
    Text,
    Help,
    Quickfix,
    Terminal,
}

impl Buftype {
    #[inline]
    fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }
}

impl FromStr for Buftype {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // `:h buftype` for more infos.
        match s {
            "" => Ok(Self::Text),
            "help" => Ok(Self::Help),
            "quickfix" => Ok(Self::Quickfix),
            "terminal" => Ok(Self::Terminal),
            other => Err(other.to_owned()),
        }
    }
}

/// Error type returned by [`TextBuffer::current`] when the current buffer
/// is not a text buffer.
#[derive(Debug)]
pub enum NotTextBufferError {
    /// The current buffer is a help file.
    Help,

    /// The current buffer is a quickfix list.
    Quickfix,

    /// The current buffer houses a terminal emulator.
    Terminal,
}
