use core::marker::PhantomData;

use crate::{Backend, BackendHandle, NeovimCtx};

/// TODO: docs.
pub struct AsyncCtx<'a, B> {
    backend: BackendHandle<B>,
    _non_static: PhantomData<&'a ()>,
}

impl<B> AsyncCtx<'_, B>
where
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn with_ctx<Fun, Out>(&self, fun: Fun) -> Out
    where
        Fun: FnOnce(&mut NeovimCtx<B>) -> Out,
    {
        self.backend.with_mut(|backend| fun(&mut NeovimCtx::new(backend)))
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(backend: BackendHandle<B>) -> Self {
        Self { backend, _non_static: PhantomData }
    }
}
