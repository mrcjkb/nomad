use std::panic;

use crate::Editor;
use crate::context::{Borrowed, Context};
use crate::module::{ApiCtx, Module, PanicInfo, Plugin};

/// A [`Plugin`] that handles panics by resuming to unwind the stack.
pub(crate) struct ResumeUnwinding;

impl<Ed: Editor> Plugin<Ed> for ResumeUnwinding {
    #[inline]
    fn handle_panic(
        &self,
        info: PanicInfo,
        _: &mut Context<Ed, Borrowed<'_>>,
    ) {
        panic::resume_unwind(info.payload);
    }
}

impl<Ed: Editor> Module<Ed> for ResumeUnwinding {
    const NAME: &str = "";
    type Config = ();

    fn api(&self, _: &mut ApiCtx<Ed>) {
        unreachable!()
    }
    fn on_new_config(
        &self,
        _: Self::Config,
        _: &mut Context<Ed, Borrowed<'_>>,
    ) {
        unreachable!()
    }
}
