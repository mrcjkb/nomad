use futures_util::stream::select;
use futures_util::StreamExt;
use nomad2::neovim::{
    command,
    function,
    CommandEvent,
    FunctionEvent,
    ModuleApi,
    Neovim,
};
use nomad2::{module_name, Context, Module, ModuleName, Subscription};

use crate::events::{JoinSession, StartSession};
use crate::{Collab, Config};

/// TODO: docs.
pub struct NeovimCollab(Collab<Neovim>);

impl Module<Neovim> for NeovimCollab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;

    fn init(ctx: &Context<Neovim>) -> (Self, ModuleApi) {
        let (join_cmd, join_cmd_sub) = command::<JoinSession>(ctx);
        let (start_cmd, start_cmd_sub) = command::<StartSession>(ctx);

        let (join_fn, join_fn_sub) = function::<JoinSession>(ctx);
        let (start_fn, start_fn_sub) = function::<StartSession>(ctx);

        let api = ModuleApi::new::<Self>()
            .with_command(join_cmd)
            .with_command(start_cmd)
            .with_function(join_fn)
            .with_function(start_fn);

        let this = Self(Collab {
            ctx: ctx.clone(),
            config: Config::default(),
            join_stream: select(join_cmd_sub, join_fn_sub),
            start_stream: select(start_cmd_sub, start_fn_sub),
        });

        (this, api)
    }

    async fn run(&mut self, _: &Context<Neovim>) {
        self.0.run().await;
    }
}
