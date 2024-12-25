use crate::executor::{BackgroundExecutor, LocalExecutor};

/// TODO: docs.
pub trait Backend: 'static {
    /// TODO: docs.
    type LocalExecutor: LocalExecutor;

    /// TODO: docs.
    type BackgroundExecutor: BackgroundExecutor;

    /// TODO: docs.
    fn init() -> Self;
}
