//! TODO: docs.

use core::marker::PhantomData;

use nvimx_core::api::{Api, ApiBuilder, ModuleApi, ModuleApiBuilder};
use nvimx_core::{Module, Plugin};

use crate::Neovim;

/// TODO: docs.
pub struct NeovimApi<P> {
    _phantom: PhantomData<P>,
}

/// TODO: docs.
pub struct NeovimModuleApi<M> {
    _phantom: PhantomData<M>,
}

impl<P> Api<P, Neovim> for NeovimApi<P>
where
    P: Plugin<Neovim>,
{
    type Builder<'a> = Self;
    type ModuleApi<M: Module<Neovim, Plugin = P>> = NeovimModuleApi<M>;
}

impl<P> ApiBuilder<NeovimApi<P>, P, Neovim> for NeovimApi<P>
where
    P: Plugin<Neovim>,
{
    #[inline]
    fn add_module<M>(&mut self, _module_api: NeovimModuleApi<M>)
    where
        M: Module<Neovim, Plugin = P>,
    {
        todo!();
    }

    #[inline]
    fn module_builder<M>(&mut self) -> &mut NeovimModuleApi<M>
    where
        M: Module<Neovim, Plugin = P>,
    {
        todo!();
    }

    #[inline]
    fn build(self) -> NeovimApi<P> {
        self
    }
}

impl<M> ModuleApi<M, Neovim> for NeovimModuleApi<M>
where
    M: Module<Neovim>,
{
    type Builder<'a> = &'a mut Self;
}

impl<M> ModuleApiBuilder<NeovimModuleApi<M>, M, Neovim>
    for &mut NeovimModuleApi<M>
where
    M: Module<Neovim>,
{
    #[inline]
    fn build(self) -> NeovimModuleApi<M> {
        todo!();
    }
}

impl<P> Default for NeovimApi<P>
where
    P: Plugin<Neovim>,
{
    #[inline]
    fn default() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<P> From<NeovimApi<P>> for crate::oxi::Dictionary
where
    P: Plugin<Neovim>,
{
    #[inline]
    fn from(_api: NeovimApi<P>) -> Self {
        todo!();
    }
}
