use std::borrow::Cow;

use crate::backend::{Backend, Buffer};

/// TODO: docs.
pub struct BufferCtx<'a, B: Backend> {
    inner: B::Buffer<'a>,
}

impl<'a, B: Backend> BufferCtx<'a, B> {
    /// TODO: docs.
    #[inline]
    pub fn id(&self) -> <B::Buffer<'a> as Buffer<B>>::Id {
        self.inner.id()
    }

    /// TODO: docs.
    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        self.inner.name()
    }

    #[inline]
    pub(crate) fn new(inner: B::Buffer<'a>) -> Self {
        Self { inner }
    }
}
