use nvimx2::{Backend, Plugin, PluginApi, PluginCtx, PluginName};

#[cfg(feature = "neovim-0-10")]
#[nvimx2::plugin(nvimx2::neovim::ZeroDotTen)]
pub fn mad() -> Mad {
    Mad
}

#[cfg(feature = "neovim-nightly")]
#[nvimx2::plugin(nvimx2::neovim::Nightly)]
pub fn mad() -> Mad {
    Mad
}

/// TODO: docs.
pub struct Mad;

impl<B: Backend> Plugin<B> for Mad {
    const NAME: &'static PluginName = PluginName::new("mad");

    type Docs = ();

    fn api(&self, _ctx: PluginCtx<'_, B>) -> PluginApi<Self, B> {
        // PluginApi::new(ctx)
        //     .with_module(auth::Auth::new())
        //     .with_module(collab::Collab::new())
        //     .with_module(version::Version::new())
        todo!();
    }

    fn docs() {}
}
