use core::fmt;

use nvim::serde::{Deserializer, Serializer};
use nvim::Object;
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::prelude::{ModuleName, WarningMsg};
use crate::warning::ChunkExt;

/// TODO: docs
pub(crate) fn deserialize<T: DeserializeOwned>(
    object: Object,
) -> Result<T, DeserializeError> {
    serde_path_to_error::deserialize(Deserializer::new(object))
        .map_err(DeserializeError::new)
}

/// TODO: docs
pub(crate) fn serialize<T: Serialize>(
    value: &T,
) -> Result<Object, SerializeError> {
    serde_path_to_error::serialize(value, Serializer::new())
        .map_err(SerializeError::new)
}

/// TODO: docs
pub(crate) struct DeserializeError {
    module_name: Option<ModuleName>,
    inner: serde_path_to_error::Error<nvim::serde::DeserializeError>,
}

impl DeserializeError {
    #[inline]
    fn new(
        err: serde_path_to_error::Error<nvim::serde::DeserializeError>,
    ) -> Self {
        Self { module_name: None, inner: err }
    }

    #[inline]
    fn path(&self) -> impl fmt::Display + '_ {
        PathToError { err: self }
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn set_module_name(&mut self, module: ModuleName) {
        self.module_name = Some(module);
    }
}

impl From<DeserializeError> for WarningMsg {
    #[inline]
    fn from(err: DeserializeError) -> WarningMsg {
        let mut msg = WarningMsg::new();

        msg.add("couldn't deserialize ")
            .add(err.path().to_string().highlight())
            .add(": ");

        use nvim::serde::DeserializeError::*;

        match err.inner.inner() {
            Custom { msg: err_msg } => {
                msg.add(err_msg.as_str());
            },

            DuplicateField { field } => {
                msg.add("duplicate field ").add(field.highlight());
            },

            MissingField { field } => {
                msg.add("missing field ").add(field.highlight());
            },

            UnknownField { field, expected } => {
                msg.add_invalid(field, expected.iter(), "field");
            },

            UnknownVariant { variant, expected } => {
                msg.add_invalid(variant, expected.iter(), "variant");
            },
        }

        msg
    }
}

struct PathToError<'a> {
    err: &'a DeserializeError,
}

impl fmt::Display for PathToError<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use nvim::serde::DeserializeError::*;

        if let Some(module_name) = &self.err.module_name {
            write!(f, "{}", module_name)?;
        }

        let segments = self.err.inner.path().iter();

        let num_segments = segments.len();

        if num_segments == 0 {
            return Ok(());
        }

        let should_print_last_segment = matches!(
            self.err.inner.inner(),
            Custom { .. } | UnknownVariant { .. }
        );

        for (idx, segment) in segments.enumerate() {
            let is_last = idx + 1 == num_segments;

            let should_print = !is_last | should_print_last_segment;

            if should_print {
                write!(f, ".{}", segment)?;
            }
        }

        Ok(())
    }
}

/// TODO: docs
pub(crate) struct SerializeError {
    inner: serde_path_to_error::Error<nvim::serde::SerializeError>,
}

impl SerializeError {
    #[inline]
    fn new(
        inner: serde_path_to_error::Error<nvim::serde::SerializeError>,
    ) -> Self {
        Self { inner }
    }
}

impl From<SerializeError> for WarningMsg {
    #[inline]
    fn from(err: SerializeError) -> WarningMsg {
        let mut msg = WarningMsg::new();

        msg.add("couldn't serialize ")
            .add(err.inner.path().to_string().highlight())
            .add(": ")
            .add(err.inner.to_string().as_str());
        msg
    }
}
