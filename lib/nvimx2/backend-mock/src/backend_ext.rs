use nvimx_core::AsyncCtx;
use nvimx_core::backend::Backend;

use crate::executor::TestExecutor;

/// TODO: docs.
pub trait BackendExt: Backend {
    /// TODO: docs.
    fn block_on<R: 'static>(
        self,
        fun: impl AsyncFnOnce(&mut AsyncCtx<Self>) -> R + 'static,
    ) -> R
    where
        Self::LocalExecutor: AsMut<TestExecutor>,
    {
        futures_lite::future::block_on(self.run(fun))
    }

    /// TODO: docs.
    fn block_on_all<R: 'static>(
        self,
        fun: impl AsyncFnOnce(&mut AsyncCtx<Self>) -> R + 'static,
    ) -> R
    where
        Self::LocalExecutor: AsMut<TestExecutor>,
    {
        futures_lite::future::block_on(self.run_all(fun))
    }

    /// TODO: docs.
    fn run<R: 'static>(
        self,
        fun: impl AsyncFnOnce(&mut AsyncCtx<Self>) -> R + 'static,
    ) -> impl Future<Output = R>
    where
        Self::LocalExecutor: AsMut<TestExecutor>,
    {
        self.run_inner(fun, false)
    }

    /// TODO: docs.
    fn run_all<R: 'static>(
        self,
        fun: impl AsyncFnOnce(&mut AsyncCtx<Self>) -> R + 'static,
    ) -> impl Future<Output = R>
    where
        Self::LocalExecutor: AsMut<TestExecutor>,
    {
        self.run_inner(fun, true)
    }

    #[doc(hidden)]
    fn run_inner<R: 'static>(
        mut self,
        fun: impl AsyncFnOnce(&mut AsyncCtx<Self>) -> R + 'static,
        run_all: bool,
    ) -> impl Future<Output = R>
    where
        Self::LocalExecutor: AsMut<TestExecutor>,
    {
        let runner = self
            .local_executor()
            .as_mut()
            .take_runner()
            .expect("runner has not been taken");

        let task = self.with_ctx(move |ctx| ctx.spawn_local_unprotected(fun));

        async move { runner.run(task, run_all).await }
    }
}

impl<B: Backend> BackendExt for B {}
