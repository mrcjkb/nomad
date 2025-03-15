use std::sync::OnceLock;

use event_listener::Event;

/// Same concept as [`OnceLock`], but allows asynchronously
/// [`wait`](Self::wait)ing for the value to be set.
pub(crate) struct AsyncOnceLock<T> {
    lock: OnceLock<T>,
    event: Event,
}

impl<T> AsyncOnceLock<T> {
    pub(crate) const fn new() -> Self {
        Self { lock: OnceLock::new(), event: Event::new() }
    }

    pub(crate) fn set(&self, value: T) -> Result<(), T> {
        self.lock.set(value).map(|()| {
            self.event.notify(usize::MAX);
        })
    }

    pub(crate) async fn wait(&self) -> &T {
        match self.lock.get() {
            Some(value) => value,
            None => {
                self.event.listen().await;
                self.lock.get().expect("the value has been set")
            },
        }
    }
}

impl<T> Default for AsyncOnceLock<T> {
    fn default() -> Self {
        Self::new()
    }
}
