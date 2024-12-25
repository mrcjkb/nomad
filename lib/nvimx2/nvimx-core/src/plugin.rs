use crate::{Backend, NeovimCtx, PluginApi};

/// TODO: docs.
pub trait Plugin<B: Backend>: 'static + Sized {
    /// TODO: docs.
    const NAME: &'static PluginName;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api(&self, ctx: PluginCtx<'_, B>) -> PluginApi<Self, B>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

/// TODO: docs.
pub struct PluginCtx<'a, B> {
    backend: &'a mut B,
}

/// TODO: docs.
#[repr(transparent)]
pub struct PluginName(str);

impl<'a, B: Backend> PluginCtx<'a, B> {
    #[doc(hidden)]
    #[inline]
    pub fn new(backend: &'a mut B) -> Self {
        Self { backend }
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
}
