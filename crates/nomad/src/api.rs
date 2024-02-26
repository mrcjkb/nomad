use crate::action::Action;

/// TODO: docs
#[derive(Default)]
pub struct Api {}

impl Api {
    /// TODO: docs
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    #[inline]
    pub fn with_function<T: Action>(self, action: T) -> Self {
        todo!();
    }
}
