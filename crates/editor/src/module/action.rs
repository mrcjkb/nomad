use crate::{Context, Editor, context};

/// TODO: docs.
pub trait Action<Ed: Editor>: 'static {
    /// TODO: docs.
    const NAME: &str;

    /// TODO: docs.
    type Args<'args>;

    /// TODO: docs.
    type Return;

    /// TODO: docs.
    fn call(
        &mut self,
        args: Self::Args<'_>,
        ctx: &mut Context<Ed, context::Borrowed<'_>>,
    ) -> Self::Return;
}
