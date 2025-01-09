use serde::de::DeserializeOwned;

use crate::NeovimCtx;
use crate::backend::Backend;
use crate::module::ApiCtx;
use crate::notify::Name;
use crate::plugin::Plugin;

/// TODO: docs.
pub trait Module<P, B>: 'static + Sized
where
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    const NAME: Name;

    /// TODO: docs.
    type Config: DeserializeOwned;

    /// TODO: docs.
    fn api(&self, ctx: &mut ApiCtx<Self, P, B>);

    /// TODO: docs.
    fn on_new_config(
        &mut self,
        new_config: Self::Config,
        ctx: &mut NeovimCtx<P, B>,
    );
}
