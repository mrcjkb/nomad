use nvimx::ctx::NeovimCtx;
use nvimx::plugin::{
    module_name,
    ConfigReceiver,
    Module,
    ModuleApi,
    ModuleName,
};

use crate::actions::{Login, Logout};

/// TODO: docs.
pub struct Auth {}

impl Module for Auth {
    const NAME: ModuleName = module_name!("auth");
    type Config = ();
    type Plugin = nomad::Nomad;

    fn init(&self, ctx: NeovimCtx<'_>) -> ModuleApi<Self> {
        let login = Login::new();
        let logout = Logout::new();

        ModuleApi::new(ctx.to_static())
            .subcommand(login.clone())
            .subcommand(logout.clone())
            .function(login)
            .function(logout)
    }

    async fn run(self, _: NeovimCtx<'static>) {}
}

impl From<ConfigReceiver<Self>> for Auth {
    fn from(_: ConfigReceiver<Self>) -> Self {
        Self {}
    }
}
