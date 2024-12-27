use neovim::Neovim;
use neovim::api::NeovimApi;
use nvimx2::{Plugin, PluginCtx, PluginName, neovim};

#[neovim::plugin]
fn mad() -> Mad {
    Mad
}

/// TODO: docs.
struct Mad;

impl Plugin<Neovim> for Mad {
    const NAME: &'static PluginName = PluginName::new("mad");

    type Docs = ();

    fn api(&self, _ctx: PluginCtx<'_, Neovim>) -> NeovimApi<Self> {
        // PluginApi::new(ctx)
        //     .with_module(auth::Auth::new())
        //     .with_module(collab::Collab::new())
        //     .with_module(version::Version::new())
        todo!();
    }

    fn docs() {}
}
