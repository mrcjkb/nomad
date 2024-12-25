use core::marker::PhantomData;

use nvimx_core::Backend;

use crate::{NeovimBackgroundExecutor, NeovimLocalExecutor, NeovimVersion};

/// TODO: docs.
pub struct Neovim<V: NeovimVersion> {
    version: PhantomData<V>,
}

impl<V: NeovimVersion> Backend for Neovim<V> {
    type LocalExecutor = NeovimLocalExecutor;
    type BackgroundExecutor = NeovimBackgroundExecutor;

    #[inline]
    fn init() -> Self {
        todo!();
    }
}
