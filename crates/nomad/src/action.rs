use crate::prelude::{ActionName, SetCtx};

/// TODO: docs
pub trait Action {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args;

    /// TODO: docs
    fn execute(&self, args: Self::Args, ctx: &mut SetCtx);
}
