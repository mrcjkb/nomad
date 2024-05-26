use nvimx_common::Apply;

/// TODO: docs.
pub struct TextBuffer {}

impl TextBuffer {
    /// TODO: docs.
    #[inline]
    pub fn current() -> Result<Self, NotTextBufferError> {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn edit<E>(&mut self, _edit: E) -> <Self as Apply<E>>::Diff
    where
        Self: Apply<E>,
    {
        todo!();
    }
}

/// TODO: docs.
#[derive(Debug)]
pub enum NotTextBufferError {}
