use core::pin::Pin;
use core::task::{Context, Poll};
use std::sync::Arc;

use async_task::Runnable;
use concurrent_queue::{ConcurrentQueue, PushError};
use nvimx_core::backend::{BackgroundExecutor, LocalExecutor, Task};

#[derive(Clone)]
pub struct TestExecutor {
    state: Arc<ConcurrentQueue<Runnable>>,
}

pin_project_lite::pin_project! {
    pub struct TestTask<T> {
        #[pin]
        inner: async_task::Task<T>,
    }
}

impl TestExecutor {
    pub fn block_on<T>(&self, _future: impl Future<Output = T>) -> T {
        todo!()
    }
}

impl LocalExecutor for TestExecutor {
    type Task<T> = TestTask<T>;

    fn spawn<Fut>(&mut self, fut: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        let this = self.clone();
        let schedule = move |runnable| match this.state.push(runnable) {
            Ok(()) => {},
            Err(PushError::Full(_)) => unreachable!("queue is unbounded"),
            Err(PushError::Closed(_)) => unreachable!("queue is never closed"),
        };
        let (runnable, task) = async_task::spawn_local(fut, schedule);
        runnable.schedule();
        TestTask { inner: task }
    }
}

impl BackgroundExecutor for TestExecutor {
    type Task<T> = TestTask<T>;

    fn spawn<Fut>(&mut self, fut: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        LocalExecutor::spawn(self, fut)
    }
}

impl Default for TestExecutor {
    fn default() -> Self {
        Self { state: Arc::new(ConcurrentQueue::unbounded()) }
    }
}

impl<T> Future for TestTask<T> {
    type Output = T;

    fn poll(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        self.project().inner.poll(ctx)
    }
}

impl<T> Task<T> for TestTask<T> {
    fn detach(self) {
        self.inner.detach();
    }
}
