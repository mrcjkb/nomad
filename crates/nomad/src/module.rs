use core::future::Future;

use serde::de::DeserializeOwned;

use crate::config::ConfigReceiver;
use crate::maybe_result::MaybeResult;
use crate::neovim::ModuleApi;
use crate::ModuleName;

/// TODO: docs.
pub trait Module: 'static + From<ConfigReceiver<Self>> {
    /// TODO: docs.
    const NAME: ModuleName;

    /// TODO: docs.
    type Config: Default + Clone + DeserializeOwned;

    /// TODO: docs.
    fn init(&self) -> ModuleApi;

    /// TODO: docs.
    fn run(self) -> impl Future<Output = impl MaybeResult<()>>;
}
