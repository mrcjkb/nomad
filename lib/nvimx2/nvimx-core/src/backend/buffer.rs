use std::borrow::Cow;

use crate::ByteOffset;
use crate::backend::Backend;

/// TODO: docs.
pub trait Buffer<B: Backend> {
    /// TODO: docs.
    fn byte_len(&self) -> ByteOffset;

    /// TODO: docs.
    fn id(&self) -> B::BufferId;

    /// TODO: docs.
    fn name(&self) -> Cow<'_, str>;
}

impl<Buf: Buffer<B>, B: Backend> Buffer<B> for &mut Buf {
    #[inline]
    fn byte_len(&self) -> ByteOffset {
        Buf::byte_len(self)
    }

    #[inline]
    fn id(&self) -> B::BufferId {
        Buf::id(self)
    }

    #[inline]
    fn name(&self) -> Cow<'_, str> {
        Buf::name(self)
    }
}
