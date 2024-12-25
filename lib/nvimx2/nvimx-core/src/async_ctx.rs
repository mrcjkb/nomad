use core::marker::PhantomData;

use crate::executor::TaskLocal;
use crate::{Backend, NeovimCtx, Shared};

/// TODO: docs.
pub struct AsyncCtx<'a, B> {
    _backend: Shared<B>,
    _non_static: PhantomData<&'a ()>,
}

impl<'a, B> AsyncCtx<'a, B>
where
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn with_ctx<F, R>(&self, f: F) -> TaskLocal<R, B>
    where
        F: FnOnce(NeovimCtx<'_, B>) -> R,
    {
        todo!();
    }
}
