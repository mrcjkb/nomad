use core::future::Future;

use serde::de::DeserializeOwned;

use crate::neovim::{ModuleApi, Neovim};
use crate::{Context, ModuleName};

/// TODO: docs.
pub trait Module: 'static + Sized {
    /// TODO: docs.
    const NAME: ModuleName;

    /// TODO: docs.
    type Config: Default + Clone + DeserializeOwned;

    /// TODO: docs.
    fn init(ctx: &Context<Neovim>) -> (Self, ModuleApi);

    /// TODO: docs.
    fn run(&mut self, ctx: &Context<Neovim>) -> impl Future<Output = ()>;
}
