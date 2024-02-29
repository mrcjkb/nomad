use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// TODO: docs
pub struct JoinHandle<T> {
    _p: core::marker::PhantomData<T>,
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<T> {
        todo!();
    }
}
