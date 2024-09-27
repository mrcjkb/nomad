use super::NeovimSpawner;
use crate::Editor;

/// TODO: docs.
#[derive(Default)]
pub struct Neovim {}

impl Editor for Neovim {
    type Spawner = NeovimSpawner;

    #[inline]
    fn spawner(&self) -> Self::Spawner {
        NeovimSpawner
    }
}
