use core::fmt;

use crate::backend::Backend;
use crate::notify;

/// TODO: docs.
pub trait Value<B: Backend>: 'static {
    /// TODO: docs.
    type MapAccess<'a>: MapAccess<B, Value = Self>;

    /// TODO: docs.
    type MapAccessError<'a>: notify::Error<B>
    where
        Self: 'a;

    /// TODO: docs.
    fn map_access(
        &mut self,
    ) -> Result<Self::MapAccess<'_>, Self::MapAccessError<'_>>;
}

/// TODO: docs.
pub trait MapAccess<B: Backend> {
    /// TODO: docs.
    type Key<'a>: Key<B>
    where
        Self: 'a;

    /// TODO: docs.
    type Value;

    /// TODO: docs.
    fn next_key(&mut self) -> Option<Self::Key<'_>>;

    /// TODO: docs.
    fn take_next_value(&mut self) -> Self::Value;
}

/// TODO: docs.
pub trait Key<B: Backend>: fmt::Debug {
    /// TODO: docs.
    type AsStrError<'a>: notify::Error<B>
    where
        Self: 'a;

    /// TODO: docs.
    fn as_str(&self) -> Result<&str, Self::AsStrError<'_>>;
}
