use core::future::Future;

use crate::action_name::ActionName;
use crate::ctx::NeovimCtx;
use crate::diagnostics::{DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::{Action, Module};

/// TODO: docs
pub trait AsyncAction: 'static {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args;

    /// TODO: docs
    type Docs;

    /// TODO: docs
    type Module: Module;

    /// TODO: docs
    fn execute(
        &mut self,
        args: Self::Args,
        ctx: NeovimCtx<'_>,
    ) -> impl Future<Output = impl MaybeResult<()>>;

    /// TODO: docs
    fn docs(&self) -> Self::Docs;
}

impl<'a, T: AsyncAction + Clone> Action<NeovimCtx<'a>> for T {
    const NAME: ActionName = T::NAME;
    type Args = T::Args;
    type Docs = T::Docs;
    type Module = T::Module;
    type Return = ();

    fn execute(&mut self, args: Self::Args, ctx: NeovimCtx<'a>) {
        let mut this = self.clone();
        let ctx_static = ctx.to_static();
        ctx.spawn(async move {
            let ctx = ctx_static.reborrow();
            if let Err(message) =
                this.execute(args, ctx).await.into_result().map_err(Into::into)
            {
                let mut source = DiagnosticSource::new();
                source
                    .push_segment(Self::Module::NAME.as_str())
                    .push_segment(Self::NAME.as_str());
                message.emit(Level::Warning, source);
            }
        })
        .detach();
    }

    fn docs(&self) -> Self::Docs {
        self.docs()
    }
}
