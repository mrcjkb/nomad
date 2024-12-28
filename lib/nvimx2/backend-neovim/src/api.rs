//! TODO: docs.

use core::marker::PhantomData;

use nvimx_core::api::{Api, ModuleApi};
use nvimx_core::{ActionName, Module, Plugin, notify};

use crate::Neovim;
use crate::oxi::{Dictionary, Function, Object};

/// TODO: docs.
pub struct NeovimApi<P> {
    dict: Dictionary,
    _phantom: PhantomData<P>,
}

/// TODO: docs.
pub struct NeovimModuleApi<'a, M: Module<Neovim>> {
    plugin_api: &'a mut NeovimApi<M::Plugin>,
    dict: Dictionary,
}

impl<P> Api<P, Neovim> for NeovimApi<P>
where
    P: Plugin<Neovim>,
{
    type ModuleApi<'a, M: Module<Neovim, Plugin = P>> = NeovimModuleApi<'a, M>;

    #[track_caller]
    #[inline]
    fn with_module<M>(&mut self) -> Self::ModuleApi<'_, M>
    where
        M: Module<Neovim, Plugin = P>,
    {
        if self.dict.get(M::NAME.as_str()).is_some() {
            panic!(
                "a module with name '{}' has already been added to {}'s API",
                M::NAME.as_str(),
                P::NAME.as_str(),
            );
        }
        NeovimModuleApi { plugin_api: self, dict: Dictionary::default() }
    }
}

impl<M> ModuleApi<M, Neovim> for NeovimModuleApi<'_, M>
where
    M: Module<Neovim>,
{
    #[track_caller]
    #[inline]
    fn add_function<Fun, Err>(&mut self, fun_name: &ActionName, mut fun: Fun)
    where
        Fun: FnMut(Object) -> Result<Object, Err> + 'static,
        Err: notify::Error,
    {
        if self.dict.get(fun_name.as_str()).is_some() {
            panic!(
                "a field with name '{}' has already been added to {}.{}'s API",
                fun_name.as_str(),
                M::Plugin::NAME.as_str(),
                M::NAME.as_str(),
            );
        }

        self.dict.insert(
            fun_name.as_str(),
            Function::from_fn_mut(move |args| fun(args).unwrap_or_default()),
        );
    }

    #[inline]
    fn finish(self) {
        self.plugin_api.dict.insert(M::NAME.as_str(), self.dict);
        todo!()
    }
}

impl<P> Default for NeovimApi<P>
where
    P: Plugin<Neovim>,
{
    #[inline]
    fn default() -> Self {
        Self { dict: Dictionary::default(), _phantom: PhantomData }
    }
}

impl<P> From<NeovimApi<P>> for Dictionary
where
    P: Plugin<Neovim>,
{
    #[inline]
    fn from(_api: NeovimApi<P>) -> Self {
        todo!();
    }
}
