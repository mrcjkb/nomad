use nvimx2::backends::{Neovim, Nightly};
use nvimx2::{Backend, Plugin, PluginApi, PluginCtx, PluginName};

#[nvimx::oxi::plugin(nvim_oxi = nvimx::oxi)]
fn nomad() -> nvimx::plugin::PluginCtx<nomad::Nomad> {
    nvimx::plugin::PluginCtx::init(nomad::Nomad)
        .with_module::<auth::Auth>()
        .with_module::<collab::Collab>()
        .with_module::<version::Version>()
}

// #[nvimx::plugin(Nightly)]
pub fn mad() -> Mad {
    Mad
}

pub struct Mad;

impl<B: Backend> Plugin<B> for Mad {
    const NAME: &'static PluginName = PluginName::new("mad");

    type Docs = ();

    fn api(&self, _ctx: PluginCtx<'_, B>) -> PluginApi<Self, B> {
        todo!();
    }

    fn docs() {}
}
