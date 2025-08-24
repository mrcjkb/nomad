use editor::command::ToCompletionFn;
use editor::context::Borrowed;
use editor::module::Action;
use editor::{Context, Editor};

use crate::VERSION;

/// TODO: docs.
#[derive(Default)]
pub struct EmitVersion {}

impl EmitVersion {
    /// Creates a new [`EmitVersion`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Ed: Editor> Action<Ed> for EmitVersion {
    const NAME: &str = "version";

    type Args<'args> = ();
    type Return = ();

    fn call(&mut self, _: Self::Args<'_>, ctx: &mut Context<Ed, Borrowed>) {
        tracing::info!(title = %ctx.namespace().dot_separated(), "{VERSION}");
    }
}

impl<Ed: Editor> ToCompletionFn<Ed> for EmitVersion {
    fn to_completion_fn(&self) {}
}
