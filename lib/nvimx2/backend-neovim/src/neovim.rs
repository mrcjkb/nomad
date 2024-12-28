use nvimx_core::{Backend, Plugin};

use crate::{api, executor, notify, oxi, serde};

/// TODO: docs.
pub struct Neovim {
    emitter: notify::NeovimEmitter,
}

impl Backend for Neovim {
    type Api<P: Plugin<Self>> = api::NeovimApi<P>;
    type ApiValue = oxi::Object;
    type LocalExecutor = executor::NeovimLocalExecutor;
    type BackgroundExecutor = executor::NeovimBackgroundExecutor;
    type Emitter<'this> = &'this mut notify::NeovimEmitter;
    type SerializeError = serde::NeovimSerializeError;
    type DeserializeError = serde::NeovimDeserializeError;

    #[inline]
    fn init() -> Self {
        Self { emitter: notify::NeovimEmitter::default() }
    }

    #[inline]
    fn api<P: Plugin<Self>>(&mut self) -> Self::Api<P> {
        api::NeovimApi::default()
    }

    #[inline]
    fn emitter(&mut self) -> Self::Emitter<'_> {
        &mut self.emitter
    }

    #[inline]
    fn serialize<T: ?Sized + ::serde::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::ApiValue, Self::SerializeError> {
        serde::serialize(value)
    }

    #[inline]
    fn deserialize<T: ::serde::de::DeserializeOwned>(
        &mut self,
        object: Self::ApiValue,
    ) -> Result<T, Self::DeserializeError> {
        serde::deserialize(object)
    }
}
