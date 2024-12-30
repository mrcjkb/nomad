use core::future::Future;

use nvimx_common::MaybeResult;
use nvimx_ctx::NeovimCtx;
use serde::de::DeserializeOwned;

use crate::ModuleName;
use crate::config::ConfigReceiver;
use crate::module_api::ModuleApi;
use crate::plugin::Plugin;

/// TODO: docs.
pub trait Module: 'static + From<ConfigReceiver<Self>> {
    /// TODO: docs.
    const NAME: ModuleName;

    /// TODO: docs.
    type Config: Default + DeserializeOwned;

    /// TODO: docs.
    type Plugin: Plugin;

    /// TODO: docs.
    fn init(&self, ctx: NeovimCtx<'_>) -> ModuleApi<Self>;

    /// TODO: docs.
    fn run(
        self,
        ctx: NeovimCtx<'static>,
    ) -> impl Future<Output = impl MaybeResult<()>>;
}
