use nvimx_common::MaybeResult;
use nvimx_ctx::NeovimCtx;
use nvimx_diagnostics::DiagnosticMessage;

use crate::action_name::ActionName;
use crate::subcommand_args::{SubCommandArgs, SubCommandCursor};
use crate::{Action, Module};

/// TODO: docs.
pub trait CompletionFunc: 'static {
    /// TODO: docs.
    #[allow(unused_variables)]
    fn call(
        &mut self,
        args: SubCommandArgs,
        cursor: SubCommandCursor,
    ) -> Vec<String> {
        Vec::new()
    }
}

fn no_completion(_: SubCommandArgs, _: SubCommandCursor) -> Vec<String> {
    Vec::new()
}

/// TODO: docs
pub trait SubCommand: 'static {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args: for<'args> TryFrom<
            SubCommandArgs<'args>,
            Error: Into<DiagnosticMessage>,
        >;

    /// TODO: docs
    type Docs;

    /// TODO: docs
    type Module: Module;

    /// TODO: docs
    fn execute<'a>(
        &'a mut self,
        args: Self::Args,
        ctx: NeovimCtx<'a>,
    ) -> impl MaybeResult<()>;

    /// TODO: docs
    fn completion_func(&self) -> impl CompletionFunc {
        no_completion
    }

    /// TODO: docs
    fn docs(&self) -> Self::Docs;
}

/// TODO: docs.
pub trait ToCompletionFunc {
    /// TODO: docs.
    fn to_completion_func(&self) -> impl CompletionFunc {
        no_completion
    }
}

impl<F> CompletionFunc for F
where
    F: FnMut(SubCommandArgs, SubCommandCursor) -> Vec<String> + 'static,
{
    fn call(
        &mut self,
        args: SubCommandArgs,
        cursor: SubCommandCursor,
    ) -> Vec<String> {
        self(args, cursor)
    }
}

impl<A> SubCommand for A
where
    A: for<'a> Action<Ctx<'a> = NeovimCtx<'a>, Return = ()> + ToCompletionFunc,
    A::Args: for<'args> TryFrom<
            SubCommandArgs<'args>,
            Error: Into<DiagnosticMessage>,
        >,
{
    const NAME: ActionName = A::NAME;
    type Args = A::Args;
    type Docs = A::Docs;
    type Module = A::Module;

    fn execute<'a>(
        &'a mut self,
        args: Self::Args,
        ctx: NeovimCtx<'a>,
    ) -> impl MaybeResult<()> {
        A::execute(self, args, ctx)
    }

    fn completion_func(&self) -> impl CompletionFunc {
        self.to_completion_func()
    }

    fn docs(&self) -> Self::Docs {
        A::docs(self)
    }
}
