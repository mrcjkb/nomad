//! TODO: docs.

use nvim_oxi::Object;
use nvimx_core::notify;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::oxi;

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct NeovimSerializeError(oxi::serde::SerializeError);

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct NeovimDeserializeError(oxi::serde::DeserializeError);

#[inline]
pub(crate) fn serialize<T: ?Sized + Serialize>(
    value: &T,
) -> Result<oxi::Object, NeovimSerializeError> {
    todo!();
}

#[inline]
pub(crate) fn deserialize<T: DeserializeOwned>(
    object: Object,
) -> Result<T, NeovimDeserializeError> {
    todo!();
}

impl notify::Error for NeovimSerializeError {
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        todo!()
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        todo!()
    }
}

impl notify::Error for NeovimDeserializeError {
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        todo!()
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        todo!()
    }
}
