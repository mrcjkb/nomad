use crate::executor::{BackgroundExecutor, LocalExecutor};
use crate::{Plugin, PluginApi, notify};

/// TODO: docs.
pub trait Backend: 'static + Sized {
    /// TODO: docs.
    type Api<P: Plugin<Self>>;

    /// TODO: docs.
    type LocalExecutor: LocalExecutor;

    /// TODO: docs.
    type BackgroundExecutor: BackgroundExecutor;

    /// TODO: docs.
    type Emitter<'a>: notify::Emitter;

    /// TODO: docs.
    fn init() -> Self;

    /// TODO: docs.
    fn emitter(&mut self) -> Self::Emitter<'_>;

    /// TODO: docs.
    fn to_backend_api<P>(
        &mut self,
        plugin_api: PluginApi<P, Self>,
    ) -> Self::Api<P>
    where
        P: Plugin<Self>;
}
