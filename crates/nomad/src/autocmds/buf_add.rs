use core::fmt;

use crate::autocmd::{Autocmd, ShouldDetach};
use crate::ctx::AutocmdCtx;
use crate::neovim::BufferId;
use crate::{Action, ActorId};

/// TODO: docs.
pub struct BufAdd<A>(pub A);

/// TODO: docs.
pub struct BufAddArgs {
    /// The [`ActorId`] of the actor that added the buffer.
    pub actor_id: ActorId,

    /// The [`BufferId`] of the buffer that was added.
    pub buffer_id: BufferId,
}

impl<A> Autocmd<A::Module> for BufAdd<A>
where
    A: Action<Args = BufAddArgs>,
    A::Docs: fmt::Display,
    A::Return: Into<ShouldDetach>,
{
    type Action = A;

    fn into_action(self) -> A {
        self.0
    }

    fn on_events(&self) -> impl IntoIterator<Item = &str> {
        ["BufAdd"]
    }
}

impl From<AutocmdCtx<'_>> for BufAddArgs {
    fn from(ctx: AutocmdCtx<'_>) -> Self {
        let buffer_id = BufferId::new(ctx.args().buffer.clone());
        let actor_id = ctx.with_actor_map(|m| m.take_added_buffer(&buffer_id));
        Self { actor_id, buffer_id }
    }
}
