use crate::ExplicitBound;

/// TODO: docs.
pub enum RequestedBound<T> {
    /// TODO: docs.
    Explicit(ExplicitBound<T>),

    /// TODO: docs.
    Unbounded,
}
