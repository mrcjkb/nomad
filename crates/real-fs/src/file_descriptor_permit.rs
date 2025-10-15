use async_lock::{Semaphore, SemaphoreGuard};

const MAX_PERMITS: usize = 64;

static FD_SEMAPHORE: Semaphore = Semaphore::new(MAX_PERMITS);

/// TODO: docs.
pub(crate) struct FileDescriptorPermit {
    _guard: SemaphoreGuard<'static>,
}

impl FileDescriptorPermit {
    /// TODO: docs.
    pub(crate) async fn acquire() -> Self {
        Self { _guard: FD_SEMAPHORE.acquire().await }
    }
}
