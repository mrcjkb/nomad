#[nvimx::oxi::plugin(nvim_oxi = nvimx::oxi)]
fn nomad() -> nvimx::plugin::PluginCtx<nomad::Nomad> {
    nvimx::plugin::PluginCtx::init(nomad::Nomad)
        .with_module::<auth::Auth>()
        .with_module::<collab::Collab>()
        .with_module::<version::Version>()
}
