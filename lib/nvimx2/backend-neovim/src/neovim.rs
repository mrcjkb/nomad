use core::marker::PhantomData;

use nvimx_core::{Backend, Plugin, PluginApi};

use crate::{
    NeovimBackgroundExecutor,
    NeovimLocalExecutor,
    NeovimVersion,
    notify,
    oxi,
};

/// TODO: docs.
pub struct Neovim<V: NeovimVersion> {
    emitter: notify::NeovimEmitter,
    version: PhantomData<V>,
}

impl<V: NeovimVersion> Backend for Neovim<V> {
    type Api<P: Plugin<Self>> = oxi::Dictionary;
    type LocalExecutor = NeovimLocalExecutor;
    type BackgroundExecutor = NeovimBackgroundExecutor;
    type Emitter<'a> = &'a mut notify::NeovimEmitter;

    #[inline]
    fn init() -> Self {
        Self {
            emitter: notify::NeovimEmitter::default(),
            version: PhantomData,
        }
    }

    #[inline]
    fn emitter(&mut self) -> Self::Emitter<'_> {
        &mut self.emitter
    }

    #[inline]
    fn to_backend_api<P>(
        &mut self,
        _plugin_api: PluginApi<P, Self>,
    ) -> Self::Api<P>
    where
        P: Plugin<Self>,
    {
        todo!();
    }
}
