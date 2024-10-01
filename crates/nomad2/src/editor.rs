use core::ops::AddAssign;

use collab_fs::Fs;

use crate::Spawner;

/// TODO: docs.
pub trait Editor: 'static {
    /// TODO: docs.
    type Api: Default + AddAssign<Self::ModuleApi>;

    /// TODO: docs.
    type ModuleApi;

    /// TODO: docs.
    type Fs: Fs;

    /// TODO: docs.
    type Spawner: Spawner;

    /// TODO: docs.
    fn fs(&self) -> Self::Fs;

    /// TODO: docs.
    fn spawner(&self) -> Self::Spawner;
}
