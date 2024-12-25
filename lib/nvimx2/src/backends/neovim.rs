use core::marker::PhantomData;

use super::NeovimVersion;

/// TODO: docs.
pub struct Neovim<V: NeovimVersion> {
    version: PhantomData<V>,
}
