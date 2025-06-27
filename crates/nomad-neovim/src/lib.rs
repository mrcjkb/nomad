//! TODO: docs.

use ed::module::{ApiCtx, Empty, Module};
use ed::notify::Name;
use ed::plugin::Plugin;
use ed::{Borrowed, Context};
use neovim::Neovim;

#[neovim::plugin]
fn nomad() -> Nomad {
    Nomad
}

struct Nomad;

impl Plugin<Neovim> for Nomad {
    const COMMAND_NAME: Name = "Mad";
}

impl Module<Neovim> for Nomad {
    const NAME: Name = "nomad";

    type Config = Empty;

    fn api(&self, ctx: &mut ApiCtx<Neovim>) {
        let auth = auth::Auth::default();
        let collab = collab::Collab::from(&auth);

        ctx.with_command(auth.login())
            .with_command(auth.logout())
            .with_command(collab.start())
            .with_command(version::EmitVersion::new())
            .with_constant(version::VERSION)
            .with_module(auth)
            .with_module(collab);
    }

    fn on_init(&self, ctx: &mut Context<Neovim, Borrowed>) {
        ctx.set_emitter(neovim::notify::detect());
    }

    fn on_new_config(
        &self,
        _: Self::Config,
        _: &mut Context<Neovim, Borrowed>,
    ) {
    }
}
