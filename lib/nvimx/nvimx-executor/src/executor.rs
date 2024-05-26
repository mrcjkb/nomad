use alloc::rc::Rc;
use core::future::Future;
use core::marker::PhantomData;

use async_task::{Builder, Runnable};
use concurrent_queue::ConcurrentQueue;
use nvim_oxi::libuv;

use crate::Task;

/// A single-threaded executor integrated with the Neovim event loop.
///
/// See the [crate-level](crate) documentation for more information.
pub struct Executor<'a> {
    /// The executor state.
    state: Rc<ExecutorState>,

    /// A handle to the callback that ticks the executor.
    callback_handle: libuv::AsyncHandle,

    /// A fake lifetime to avoid having to require a `'static` lifetime for the
    /// futures given to [`spawn`](Self::spawn).
    _lifetime: PhantomData<&'a ()>,
}

struct ExecutorState {
    /// The queue of tasks that are ready to be polled.
    woken_queue: ConcurrentQueue<Runnable<()>>,
}

impl<'a> Executor<'a> {
    /// TODO: docs.
    ///
    /// # Panics
    ///
    /// Panics if called from a non-main thread.
    #[inline]
    pub fn register() -> Self {
        // TODO: assert that it's the main thread.

        let state = Rc::new(ExecutorState::new());

        let callback_handle = {
            let state = Rc::clone(&state);

            libuv::AsyncHandle::new(move || {
                let state = Rc::clone(&state);
                // We schedule the poll to avoid `textlock` and other
                // synchronization issues.
                nvim_oxi::schedule(move |_| {
                    state.poll_all_woken();
                    Ok(())
                });
                Ok::<_, core::convert::Infallible>(())
            })
            .expect("never fails(?)")
        };

        Self { state, callback_handle, _lifetime: PhantomData }
    }

    /// TODO: docs.
    #[inline]
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future<Output = ()> + 'a,
    {
        let builder = Builder::new().propagate_panic(true);

        let schedule = {
            let callback_handle = self.callback_handle.clone();
            let state = Rc::clone(&self.state);
            move |runnable| {
                state.woken_queue.push(runnable).expect("unbounded queue");
                callback_handle.send().expect("never fails(?)");
            }
        };

        // SAFETY:
        // - future outlives the executor;
        // - runnables are dropped when `ExecutorState::poll_all_woken` is
        //   called, and `Self::register` made sure that happens on the main
        //   thread.
        let (runnable, task) =
            unsafe { builder.spawn_unchecked(|_| future, schedule) };

        runnable.schedule();

        Task::new(task)
    }
}

impl ExecutorState {
    /// Creates a new [`ExecutorState`].
    #[inline]
    fn new() -> Self {
        Self { woken_queue: ConcurrentQueue::unbounded() }
    }

    /// Polls all the tasks that have awoken since the last poll.
    ///
    /// This consumes the task queue in a FIFO manner.
    #[inline]
    fn poll_all_woken(&self) {
        while let Ok(runnable) = self.woken_queue.pop() {
            runnable.run();
        }
    }
}
