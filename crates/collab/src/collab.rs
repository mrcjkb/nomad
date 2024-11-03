use nomad::config::ConfigReceiver;
use nomad::ctx::NeovimCtx;
use nomad::{module_name, Module, ModuleApi, ModuleName, Shared};

use crate::actions::{Join, Start};
use crate::config::Config;
use crate::session_status::SessionStatus;

/// TODO: docs.
pub struct Collab {
    config: Config,
    config_rx: ConfigReceiver<Self>,
    session_status: Shared<SessionStatus>,
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;

    fn init(&self, ctx: NeovimCtx<'_>) -> ModuleApi<Self> {
        let join = Join::new(self.session_status.clone());
        let start = Start::new(self.session_status.clone());

        ModuleApi::new(ctx.to_static())
            .command(join.clone())
            .command(start.clone())
            .function(join)
            .function(start)
    }

    async fn run(mut self, _: NeovimCtx<'static>) {
        loop {
            self.config = self.config_rx.recv().await;
        }
    }
}

impl From<ConfigReceiver<Self>> for Collab {
    fn from(config_rx: ConfigReceiver<Self>) -> Self {
        Self {
            config: Config::default(),
            config_rx,
            session_status: Shared::default(),
        }
    }
}
