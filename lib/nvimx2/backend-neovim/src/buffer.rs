use std::borrow::Cow;
use std::path::PathBuf;

use nvimx_core::backend::Buffer;

use crate::Neovim;

/// TODO: docs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NeovimBuffer(crate::oxi::BufHandle);

impl NeovimBuffer {
    /// Returns this buffer's handle.
    #[inline]
    pub fn handle(&self) -> crate::oxi::BufHandle {
        self.0
    }

    #[inline]
    pub(crate) fn current() -> Self {
        Self::new(crate::oxi::api::Buffer::current())
    }

    #[inline]
    pub(crate) fn exists(&self) -> bool {
        self.inner().is_valid()
    }

    #[inline]
    pub(crate) fn get_name(&self) -> PathBuf {
        debug_assert!(self.exists());
        self.inner().get_name().expect("buffer exists")
    }

    #[inline]
    fn inner(&self) -> crate::oxi::api::Buffer {
        self.handle().into()
    }

    #[inline]
    fn new(inner: crate::oxi::api::Buffer) -> Self {
        Self(inner.handle())
    }
}

impl Buffer<Neovim> for NeovimBuffer {
    type Id = Self;

    #[inline]
    fn id(&self) -> Self::Id {
        *self
    }

    #[inline]
    fn name(&self) -> Cow<'_, str> {
        self.get_name().to_string_lossy().into_owned().into()
    }
}

#[cfg(feature = "mlua")]
impl crate::oxi::mlua::IntoLua for NeovimBuffer {
    #[inline]
    fn into_lua(
        self,
        lua: &crate::oxi::mlua::Lua,
    ) -> crate::oxi::mlua::Result<crate::oxi::mlua::Value> {
        self.handle().into_lua(lua)
    }
}
