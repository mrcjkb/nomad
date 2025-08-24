use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::module::Action;
use crate::{Context, Editor, context};

/// TODO: docs.
pub trait Function<Ed: Editor>: 'static {
    /// TODO: docs.
    const NAME: &str;

    /// TODO: docs.
    type Args<'args>: Deserialize<'args>;

    /// TODO: docs.
    type Return: Serialize;

    /// TODO: docs.
    fn call(
        &mut self,
        args: Self::Args<'_>,
        ctx: &mut Context<Ed, context::Borrowed<'_>>,
    ) -> Self::Return;
}

impl<A, Ed> Function<Ed> for A
where
    A: Action<Ed>,
    for<'args> A::Args<'args>: Deserialize<'args>,
    A::Return: Serialize,
    Ed: Editor,
{
    const NAME: &str = A::NAME;

    type Args<'a> = A::Args<'a>;
    type Return = A::Return;

    #[inline]
    fn call(
        &mut self,
        args: A::Args<'_>,
        ctx: &mut Context<Ed, context::Borrowed<'_>>,
    ) -> Self::Return {
        A::call(self, args, ctx)
    }
}
