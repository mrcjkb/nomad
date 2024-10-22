use crate::autocmd::{AutoCommand, AutoCommandEvent, ShouldDetach};
use crate::ctx::AutoCommandCtx;
use crate::neovim::BufferId;
use crate::{Action, ActorId};

/// TODO: docs.
pub struct BufAdd<A> {
    action: A,
}

/// TODO: docs.
pub struct BufAddArgs {
    /// The [`ActorId`] of the actor that added the buffer.
    pub actor_id: ActorId,

    /// The [`BufferId`] of the buffer that was added.
    pub buffer_id: BufferId,
}

impl<A> BufAdd<A> {
    /// Creates a new [`BufAdd`] with the given action.
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A> AutoCommand for BufAdd<A>
where
    A: Action<Args = BufAddArgs> + Clone,
    A::Return: Into<ShouldDetach>,
{
    type Action = A;

    fn into_action(self) -> Self::Action {
        self.action
    }

    fn on_events(&self) -> impl IntoIterator<Item = AutoCommandEvent> {
        [AutoCommandEvent::BufAdd]
    }

    fn take_actor_id(ctx: &AutoCommandCtx<'_>) -> ActorId {
        let buffer_id = BufferId::new(ctx.args().buffer.clone());
        ctx.with_actor_map(|m| m.take_added_buffer(&buffer_id))
    }
}

impl From<(ActorId, &AutoCommandCtx<'_>)> for BufAddArgs {
    fn from((actor_id, ctx): (ActorId, &AutoCommandCtx<'_>)) -> Self {
        Self { actor_id, buffer_id: BufferId::new(ctx.args().buffer.clone()) }
    }
}
