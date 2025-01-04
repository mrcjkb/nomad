use core::pin::Pin;
use core::task::{Context, Poll};

use nvimx_core::executor::{BackgroundExecutor, Task};

/// TODO: docs.
pub struct NeovimBackgroundExecutor;

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct NeovimBackgroundTask<T> {
        #[pin]
        inner: async_task::Task<T>,
    }
}

impl BackgroundExecutor for NeovimBackgroundExecutor {
    type Task<T> = NeovimBackgroundTask<T>;

    #[inline]
    fn spawn<Fut>(&mut self, _fut: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + Sync + 'static,
        Fut::Output: Send + Sync + 'static,
    {
        todo!();
    }
}

impl<T> Task<T> for NeovimBackgroundTask<T> {
    #[inline]
    fn detach(self) {
        todo!();
    }
}

impl<T> Future for NeovimBackgroundTask<T> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<T> {
        todo!();
    }
}
