use core::future::Future;

use super::JoinHandle;

struct Executor {}

/// TODO: doc
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    todo!();
}
