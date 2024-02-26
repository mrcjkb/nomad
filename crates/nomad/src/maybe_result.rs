use std::error::Error as StdError;

/// TODO: docs
pub trait MaybeResult<T> {}

impl MaybeResult<()> for () {}

impl<T, E> MaybeResult<T> for Result<T, E> where E: StdError {}
