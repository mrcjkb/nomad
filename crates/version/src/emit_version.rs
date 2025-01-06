use nvimx2::command::ToCompletionFn;
use nvimx2::notify::Message;
use nvimx2::{Action, ActionCtx, Backend, Name};

use crate::VERSION;

/// TODO: docs.
#[derive(Default)]
pub struct EmitVersion {}

impl EmitVersion {
    /// Creates a new [`EmitVersion`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

// FIXME: why does implementing `Command` cause a `conflicting implementations`
// error?
impl<B: Backend> Action<B> for EmitVersion {
    const NAME: Name = "version";

    type Args = ();
    type Return = ();

    fn call(&mut self, _: Self::Args, ctx: &mut ActionCtx<B>) {
        ctx.emit_info(Message::from_debug(VERSION));
    }
}

impl<B: Backend> ToCompletionFn<B> for EmitVersion {
    fn to_completion_fn(&self) {}
}
