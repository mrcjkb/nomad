use std::borrow::Cow;

use crate::backend::Backend;

/// TODO: docs.
pub trait Buffer<B: Backend> {
    /// TODO: docs.
    fn id(&self) -> B::BufferId;

    /// TODO: docs.
    fn name(&self) -> Cow<'_, str>;
}
