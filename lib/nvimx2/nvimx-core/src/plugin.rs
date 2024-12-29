use core::marker::PhantomData;

use crate::api::{Api, ModuleApi};
use crate::{ActionName, Backend, BackendHandle, Module, ModuleApiCtx};

/// TODO: docs.
pub trait Plugin<B: Backend>: 'static + Sized {
    /// TODO: docs.
    const NAME: &'static PluginName;

    /// TODO: docs.
    const COMMAND_NAME: &'static ActionName =
        ActionName::new(Self::NAME.uppercase_first().as_str());

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api(&self, ctx: PluginApiCtx<'_, Self, B>) -> B::Api<Self>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

/// TODO: docs.
pub struct PluginApiCtx<'a, P: Plugin<B>, B: Backend> {
    api: B::Api<P>,
    backend: BackendHandle<B>,
    _phantom: PhantomData<&'a ()>,
}

/// TODO: docs.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PluginName(str);

impl<P, B> PluginApiCtx<'_, P, B>
where
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn into_api(self) -> B::Api<P> {
        self.api
    }

    /// TODO: docs.
    #[track_caller]
    #[inline]
    pub fn with_module<M>(mut self, module: M) -> Self
    where
        M: Module<B, Plugin = P>,
    {
        module.api(ModuleApiCtx::new(&mut self.api, &self.backend));
        self
    }

    #[doc(hidden)]
    pub fn new(backend: B) -> Self {
        let backend = BackendHandle::new(backend);
        let api = backend.with_mut(|mut b| B::api::<P>(&mut b));
        Self { api, backend, _phantom: PhantomData }
    }
}

impl PluginName {
    /// TODO: docs.
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /// TODO: docs.
    #[inline]
    pub const fn new(name: &str) -> &Self {
        assert!(!name.is_empty());
        assert!(name.len() <= 24);
        // SAFETY: `PluginName` is a `repr(transparent)` newtype around `str`.
        unsafe { &*(name as *const str as *const Self) }
    }

    /// TODO: docs.
    #[inline]
    pub const fn uppercase_first(&self) -> &Self {
        todo!();
    }
}
