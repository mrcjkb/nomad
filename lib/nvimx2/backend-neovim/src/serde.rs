//! TODO: docs.

use nvimx_core::notify;
use serde::Serialize;
use serde::de::DeserializeOwned;
use smallvec::SmallVec;

use crate::oxi;
use crate::value::NeovimValue;

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct NeovimSerializeError {
    inner: serde_path_to_error::Error<oxi::serde::SerializeError>,
}

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct NeovimDeserializeError {
    inner: serde_path_to_error::Error<oxi::serde::DeserializeError>,
}

struct Path<'a> {
    inner: &'a serde_path_to_error::Path,
}

#[inline]
pub(crate) fn serialize<T: ?Sized + Serialize>(
    value: &T,
) -> Result<NeovimValue, NeovimSerializeError> {
    serde_path_to_error::serialize(value, oxi::serde::Serializer::new())
        .map(NeovimValue::new)
        .map_err(|inner| NeovimSerializeError { inner })
}

#[inline]
pub(crate) fn deserialize<T: DeserializeOwned>(
    value: NeovimValue,
) -> Result<T, NeovimDeserializeError> {
    serde_path_to_error::deserialize(oxi::serde::Deserializer::new(
        value.into_inner(),
    ))
    .map_err(|inner| NeovimDeserializeError { inner })
}

impl NeovimSerializeError {
    #[inline]
    fn path(&self) -> Path<'_> {
        Path { inner: self.inner.path() }
    }
}

impl NeovimDeserializeError {
    #[inline]
    fn path(&self) -> Path<'_> {
        Path { inner: self.inner.path() }
    }
}

impl Path<'_> {
    /// If the path is not empty, pushes " at {self}" to the given message.
    #[inline]
    pub(crate) fn push_at(&self, message: &mut notify::Message) {
        if self.inner.iter().len() > 1 {
            message.push_str(" at ").push_info(self.inner.to_string());
        }
    }
}

impl notify::Error for NeovimSerializeError {
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        Some(notify::Level::Error)
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        let mut message = notify::Message::new();
        message
            .push_str("couldn't serialize value")
            .push_with(|message| self.path().push_at(message))
            .push_str(": ")
            .push_str(self.inner.inner().to_string());
        message
    }
}

impl notify::Error for NeovimDeserializeError {
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        Some(notify::Level::Error)
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        let mut message = notify::Message::new();
        message
            .push_str("couldn't deserialize value")
            .push_with(|message| self.path().push_at(message))
            .push_str(": ");

        let (actual, &expected) = match self.inner.inner() {
            oxi::serde::DeserializeError::Custom { msg } => {
                message.push_str(msg);
                return message;
            },
            oxi::serde::DeserializeError::DuplicateField { field } => {
                message.push_str("duplicate field ").push_info(field);
                return message;
            },
            oxi::serde::DeserializeError::MissingField { field } => {
                message.push_str("missing field ").push_info(field);
                return message;
            },
            oxi::serde::DeserializeError::UnknownField { field, expected } => {
                message
                    .push_str("invalid field ")
                    .push_invalid(field)
                    .push_str(", ");
                (field, expected)
            },
            oxi::serde::DeserializeError::UnknownVariant {
                variant,
                expected,
            } => {
                message
                    .push_str("invalid variant ")
                    .push_invalid(variant)
                    .push_str(", ");
                (variant, expected)
            },
        };

        let levenshtein_threshold = 2;

        let mut guesses = expected
            .into_iter()
            .map(|candidate| {
                let distance = strsim::levenshtein(candidate, actual);
                (candidate, distance)
            })
            .filter(|&(_, distance)| distance <= levenshtein_threshold)
            .collect::<SmallVec<[_; 2]>>();

        guesses.sort_by_key(|&(_, distance)| distance);

        if let Some((best_guess, _)) = guesses.first() {
            message
                .push_str("did you mean ")
                .push_expected(best_guess)
                .push_str("?");
        } else {
            message
                .push_str("expected one of ")
                .push_comma_separated(expected, notify::SpanKind::Expected);
        }

        message
    }
}
