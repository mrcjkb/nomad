use core::future::Future;
use core::time::Duration;

use futures_util::{pin_mut, select, FutureExt, Stream, StreamExt};

use crate::clear::Clear;
use crate::clear_after::ClearAfter;
use crate::Emit;

/// TODO: docs.
pub trait EmitExt {
    /// TODO: docs.
    fn clear_after(self, duration: Duration) -> ClearAfter<Self>
    where
        Self: Emit;

    /// TODO: docs.
    fn emitting<T>(self, stream: T) -> impl Future<Output = Self::Output>
    where
        Self: Future + Sized,
        T: Stream,
        T::Item: Emit;
}

impl<T> EmitExt for T {
    fn clear_after(self, duration: Duration) -> ClearAfter<Self>
    where
        Self: Emit,
    {
        ClearAfter::new(self, duration)
    }

    fn emitting<S>(
        self,
        stream: S,
    ) -> impl Future<Output = <Self as Future>::Output>
    where
        Self: Future + Sized,
        S: Stream,
        S::Item: Emit,
    {
        let future = self;

        async move {
            pin_mut!(future);
            pin_mut!(stream);

            let mut maybe_stream = Some(stream);
            let output = loop {
                match &mut maybe_stream {
                    Some(stream) => {
                        select! {
                            output = (&mut future).fuse() => break output,
                            item = stream.next().fuse() => {
                                if let Some(item) = item {
                                    item.emit();
                                } else {
                                    maybe_stream = None;
                                }
                            }
                        }
                    },
                    None => break future.await,
                }
            };
            Clear.emit();
            output
        }
    }
}
