use core::str::FromStr;

use api::opts::OptionOpts;
use nvim_oxi::api::{self, Buffer};
use nvimx_common::Apply;

/// TODO: docs.
pub struct TextBuffer {
    /// TODO: docs.
    _attach_status: AttachStatus,

    /// TODO: docs.
    _inner: Buffer,
}

impl TextBuffer {
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
        Self { _attach_status: AttachStatus::NotAttached, _inner: inner }
    }
}

enum AttachStatus {
    Attached,
    NotAttached,
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
