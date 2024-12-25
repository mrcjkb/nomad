use crate::{
    Action,
    ActionName,
    Backend,
    CommandArgs,
    MaybeResult,
    Module,
    NeovimCtx,
};

/// TODO: docs.
pub trait Command<B: Backend>: 'static {
    /// TODO: docs.
    const NAME: &'static ActionName;

    /// TODO: docs.
    type Module: Module<B>;

    /// TODO: docs.
    type Args: for<'a> TryFrom<CommandArgs<'a>>;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn call(
        &mut self,
        args: Self::Args,
        ctx: NeovimCtx<'_, B>,
    ) -> impl MaybeResult<()>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

impl<A, B> Command<B> for A
where
    A: Action<B, Return = ()>,
    A::Args: for<'a> TryFrom<CommandArgs<'a>>,
    B: Backend,
{
    const NAME: &'static ActionName = A::NAME;

    type Module = A::Module;
    type Args = A::Args;
    type Docs = A::Docs;

    #[inline]
    fn call(
        &mut self,
        args: Self::Args,
        ctx: NeovimCtx<'_, B>,
    ) -> impl MaybeResult<()> {
        A::call(self, args, ctx)
    }

    #[inline]
    fn docs() -> Self::Docs {
        A::docs()
    }
}
