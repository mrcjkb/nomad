use nvimx_core::AsyncCtx;
use nvimx_core::backend::Backend;

use crate::executor::TestExecutor;

/// TODO: docs.
pub trait BackendExt: Backend {
    /// TODO: docs.
    fn block_on<R>(
        mut self,
        fun: impl AsyncFnOnce(&mut AsyncCtx<Self>) -> R,
    ) -> R
    where
        Self::LocalExecutor: AsRef<TestExecutor>,
    {
        self.local_executor()
            .as_ref()
            .clone()
            .block_on(self.with_async_ctx(fun))
    }
}

impl<B: Backend> BackendExt for B {}
