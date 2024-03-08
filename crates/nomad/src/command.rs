use crate::action::Action;
use crate::module::Module;

/// TODO: docs
pub struct Command {}

impl<M: Module, T: Action<M>> From<T> for Command {
    #[inline]
    fn from(_action: T) -> Self {
        Self {}
    }
}
