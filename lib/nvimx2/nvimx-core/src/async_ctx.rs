use core::marker::PhantomData;

use crate::backend::{Backend, BackgroundExecutor, TaskBackground};
use crate::notify::{Name, Namespace};
use crate::state::StateHandle;
use crate::{NeovimCtx, notify};

/// TODO: docs.
pub struct AsyncCtx<'a, B> {
    namespace: Namespace,
    state: StateHandle<B>,
    _non_static: PhantomData<&'a ()>,
}

impl<B: Backend> AsyncCtx<'_, B> {
    /// TODO: docs.
    #[inline]
    pub fn emit_err<Err>(&mut self, err: Err)
    where
        Err: notify::Error,
    {
        self.emit_err_inner(None, err);
    }

    /// TODO: docs.
    #[inline]
    pub fn spawn_background<Fut>(
        &self,
        fut: Fut,
    ) -> TaskBackground<Fut::Output, B>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        let task = self
            .state
            .with_mut(|mut state| state.background_executor().spawn(fut));
        TaskBackground::new(task)
    }

    /// TODO: docs.
    #[inline]
    pub fn with_ctx<Fun, Out>(&self, fun: Fun) -> Out
    where
        Fun: FnOnce(&mut NeovimCtx<B>) -> Out,
    {
        self.state.with_mut(|mut state| state.with_ctx(&self.namespace, fun))
    }

    #[inline]
    pub(crate) fn emit_err_inner<Err>(
        &mut self,
        action_name: Option<Name>,
        err: Err,
    ) where
        Err: notify::Error,
    {
        self.with_ctx(move |ctx| ctx.emit_err(action_name, err));
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(namespace: Namespace, state: StateHandle<B>) -> Self {
        Self { namespace, state, _non_static: PhantomData }
    }
}
