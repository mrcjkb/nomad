use crate::{Backend, Module, Plugin};

/// TODO: docs.
pub struct PluginApi<P, B> {
    plugin: P,
    backend: B,
}

impl<P, B> PluginApi<P, B>
where
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn with_default_module<M>(self, module: M) -> Self
    where
        M: Module<B, Plugin = P>,
    {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn with_module<M>(self, module: M) -> Self
    where
        M: Module<B, Plugin = P>,
    {
        todo!();
    }
}
