use std::borrow::Cow;

use crate::backend::Backend;

/// TODO: docs.
pub trait Buffer<B: Backend> {
    /// TODO: docs.
    type Id: Clone;

    /// TODO: docs.
    fn id(&self) -> Self::Id;

    /// TODO: docs.
    fn name(&self) -> Cow<'_, str>;
}
