use crate::autocmd::{AutoCommand, AutoCommandEvent, ShouldDetach};
use crate::ctx::AutoCommandCtx;
use crate::neovim::BufferId;
use crate::{Action, ActorId};

/// TODO: docs.
pub struct BufUnload<A> {
    action: A,
}

/// TODO: docs.
pub struct BufUnloadArgs {
    /// The [`ActorId`] of the actor that unloaded the buffer.
    pub actor_id: ActorId,

    /// The [`BufferId`] of the buffer that was unloaded.
    pub buffer_id: BufferId,
}

impl<A> BufUnload<A> {
    /// Creates a new [`BufUnload`] with the given action.
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A> AutoCommand for BufUnload<A>
where
    A: Action<Args = BufUnloadArgs> + Clone,
    A::Return: Into<ShouldDetach>,
{
    type Action = A;

    fn into_action(self) -> Self::Action {
        self.action
    }

    fn on_events(&self) -> impl IntoIterator<Item = AutoCommandEvent> {
        [AutoCommandEvent::BufUnload]
    }

    fn take_actor_id(_: &AutoCommandCtx<'_>) -> ActorId {
        ActorId::unknown()
    }
}

impl From<(ActorId, &AutoCommandCtx<'_>)> for BufUnloadArgs {
    fn from((actor_id, _): (ActorId, &AutoCommandCtx<'_>)) -> Self {
        Self { actor_id, buffer_id: BufferId::current() }
    }
}
