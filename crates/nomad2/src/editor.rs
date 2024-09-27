use crate::Spawner;

/// TODO: docs.
pub trait Editor: 'static {
    /// TODO: docs.
    type Spawner: Spawner;

    /// TODO: docs.
    fn spawner(&self) -> Self::Spawner;
}
