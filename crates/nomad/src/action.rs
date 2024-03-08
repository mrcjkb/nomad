use serde::de::DeserializeOwned;

use crate::prelude::{ActionName, Module, SetCtx};

/// TODO: docs
pub trait Action<M: Module>: 'static {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args: DeserializeOwned;

    /// TODO: docs
    fn execute(&self, args: Self::Args, ctx: &mut SetCtx);
}
