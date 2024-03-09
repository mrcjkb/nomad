//! TODO: docs

use std::error::Error as StdError;

/// TODO: docs
pub trait MaybeResult<T> {
    /// TODO: docs
    type Error: StdError;

    /// TODO: docs
    fn into_result(self) -> Result<T, Self::Error>;
}

impl<T> MaybeResult<T> for T {
    // TODO: change this to the never type (!) when it becomes stable.
    type Error = core::convert::Infallible;

    #[inline]
    fn into_result(self) -> Result<T, Self::Error> {
        Ok(self)
    }
}

impl<T, E> MaybeResult<T> for Result<T, E>
where
    E: StdError,
{
    type Error = E;

    #[inline]
    fn into_result(self) -> Result<T, Self::Error> {
        self
    }
}
