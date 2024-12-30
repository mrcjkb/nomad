use nvimx::Shared;
use nvimx::ctx::NeovimCtx;
use nvimx::plugin::{
    ConfigReceiver,
    Module,
    ModuleApi,
    ModuleName,
    module_name,
};

use crate::actions::{Join, Start, Yank};
use crate::config::Config;
use crate::session_status::SessionStatus;

/// TODO: docs.
pub struct Collab {
    pub(crate) config: Shared<Config>,
    pub(crate) config_rx: ConfigReceiver<Self>,
    pub(crate) session_status: Shared<SessionStatus>,
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;
    type Plugin = nomad::Nomad;

    fn init(&self, ctx: NeovimCtx<'_>) -> ModuleApi<Self> {
        let join = Join::from(self);
        let start = Start::from(self);
        let yank = Yank::new(self.session_status.clone());

        ModuleApi::new(ctx.to_static())
            .subcommand(join.clone())
            .subcommand(start.clone())
            .subcommand(yank.clone())
            .function(join)
            .function(start)
            .function(yank)
    }

    async fn run(mut self, _: NeovimCtx<'static>) {
        loop {
            self.config.set(self.config_rx.recv().await);
        }
    }
}

impl From<ConfigReceiver<Self>> for Collab {
    fn from(config_rx: ConfigReceiver<Self>) -> Self {
        Self {
            config: Shared::default(),
            config_rx,
            session_status: Shared::default(),
        }
    }
}
