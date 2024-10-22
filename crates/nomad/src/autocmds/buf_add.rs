use crate::autocmd::{Autocmd, AutocmdCallback, AutocmdEvent, ShouldDetach};
use crate::ctx::AutocmdCtx;
use crate::maybe_result::MaybeResult;
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

impl<A> Autocmd for BufAdd<A>
where
    A: Action<Args = BufAddArgs> + Clone,
    A::Return: Into<ShouldDetach>,
{
    fn into_callback(self) -> impl AutocmdCallback + Clone + 'static {
        let mut action = self.0;
        move |actor_id, ctx| {
            let buffer_id = BufferId::new(ctx.args().buffer.clone());
            let args = BufAddArgs { actor_id, buffer_id };
            action
                .execute(args)
                .into_result()
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    fn on_events(&self) -> impl IntoIterator<Item = AutocmdEvent> {
        [AutocmdEvent::BufAdd]
    }

    fn take_actor_id(ctx: &AutocmdCtx<'_>) -> ActorId {
        let buffer_id = BufferId::new(ctx.args().buffer.clone());
        ctx.with_actor_map(|m| m.take_added_buffer(&buffer_id))
    }
}
