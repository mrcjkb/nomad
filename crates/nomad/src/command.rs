use crate::action::Action;

/// TODO: docs
pub struct Command {}

impl<T: Action> From<T> for Command {
    #[inline]
    fn from(_action: T) -> Self {
        todo!();
    }
}
