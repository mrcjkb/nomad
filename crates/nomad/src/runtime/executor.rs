use core::cell::{OnceCell, UnsafeCell};
use core::future::Future;

use super::JoinHandle;

thread_local! {
    static EXECUTOR: OnceCell<UnsafeCell<LocalExecutor>>
        = const { OnceCell::new() };
}

/// TODO: doc
#[inline]
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    with_executor(move |executor| executor.spawn(future))
}

/// TODO: docs
#[inline]
fn with_executor<F, R>(fun: F) -> R
where
    F: FnOnce(&mut LocalExecutor) -> R,
{
    let executor_ptr = EXECUTOR
        .with(|executor| executor.get_or_init(UnsafeCell::default).get());

    // SAFETY: we never give out references to the executor, but can we prove
    // that the function is not reentrant?
    let executor = unsafe { &mut *executor_ptr };

    fun(executor)
}

/// TODO: docs
#[derive(Default)]
struct LocalExecutor {}

impl LocalExecutor {
    /// TODO: docs
    #[inline]
    fn spawn<F: Future>(&mut self, _future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        todo!();
    }
}
