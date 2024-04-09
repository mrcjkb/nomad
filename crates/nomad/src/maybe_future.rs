//! TODO: docs

use core::future::{ready, Future};
use core::pin::{pin, Pin};
use core::task::{Context, Poll};

use pin_project::pin_project;

/// TODO: docs
pub trait MaybeFuture: Sized {
    /// TODO: docs
    type Output;

    /// TODO: docs
    fn into_enum(
        self,
    ) -> MaybeFutureEnum<impl Future<Output = Self::Output>, Self::Output>;

    /// TODO: docs
    #[inline]
    fn into_future(self) -> impl Future<Output = Self::Output> {
        MaybeFutureFuture::from(self.into_enum())
    }

    /// TODO: docs
    #[track_caller]
    #[inline]
    fn into_ready(self) -> Self::Output {
        match self.into_enum() {
            MaybeFutureEnum::Ready(output) => output,
            MaybeFutureEnum::Future(_) => panic!("future is not ready"),
        }
    }
}

/// TODO: docs
pub enum MaybeFutureEnum<F: Future<Output = T>, T> {
    /// TODO: docs
    Ready(T),

    /// TODO: docs
    Future(F),
}

impl<F, T> MaybeFuture for MaybeFutureEnum<F, T>
where
    F: Future<Output = T>,
{
    type Output = T;

    #[inline]
    fn into_enum(self) -> MaybeFutureEnum<F, T> {
        self
    }
}

impl<F, T> From<F> for MaybeFutureEnum<F, T>
where
    F: Future<Output = T>,
{
    #[inline]
    fn from(future: F) -> Self {
        MaybeFutureEnum::Future(future)
    }
}

#[pin_project(project = MaybeFutureFutureProj)]
enum MaybeFutureFuture<F: Future<Output = T>, T> {
    Ready(core::future::Ready<T>),
    Future(#[pin] F),
}

impl<F, T> From<MaybeFutureEnum<F, T>> for MaybeFutureFuture<F, T>
where
    F: Future<Output = T>,
{
    #[inline]
    fn from(future: MaybeFutureEnum<F, T>) -> Self {
        match future {
            MaybeFutureEnum::Ready(output) => Self::Ready(ready(output)),
            MaybeFutureEnum::Future(future) => Self::Future(future),
        }
    }
}

impl<F, T> Future for MaybeFutureFuture<F, T>
where
    F: Future<Output = T>,
{
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            MaybeFutureFutureProj::Ready(future) => Pin::new(future).poll(cx),
            MaybeFutureFutureProj::Future(future) => future.poll(cx),
        }
    }
}

impls::ready!(());
impls::ready!(T; Option<T>);
impls::ready!(T, E; Result<T, E>);

mod impls {
    /// ..
    macro_rules! ready {
        ($ty:ty) => {
            impl MaybeFuture for $ty {
                type Output = Self;

                #[inline]
                fn into_enum(self) -> MaybeFutureEnum<::core::future::Ready<Self>, Self> {
                    MaybeFutureEnum::Ready(self)
                }
            }
        };

        ($($gen:ident),*; $ty:ty) => {
            impl<$($gen),*> MaybeFuture for $ty {
                type Output = Self;

                #[inline]
                fn into_enum(self) -> MaybeFutureEnum<::core::future::Ready<Self>, Self> {
                    MaybeFutureEnum::Ready(self)
                }
            }
        };
    }

    pub(super) use ready;
}
