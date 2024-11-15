use core::marker::PhantomData;
use core::ops::Deref;

use nvimx_action::{Action, ActionName, IntoModuleName};
use nvimx_common::MaybeResult;
use nvimx_ctx::{
    ActorId,
    AutoCommand,
    AutoCommandCtx,
    AutoCommandEvent,
    BufferId,
    NeovimCtx,
    ShouldDetach,
};

/// TODO: docs.
pub struct BufLeave<A, M> {
    action: BufLeaveAction<A, M>,
    buffer_id: Option<BufferId>,
}

/// TODO: docs.
#[derive(Debug, Copy, Clone)]
pub struct BufLeaveArgs {
    /// The [`ActorId`] that focused the buffer.
    pub actor_id: ActorId,

    /// The [`BufferId`] of the buffer that was left.
    pub old_buffer_id: BufferId,
}

pub struct BufLeaveAction<A, M> {
    action: A,
    module_name: PhantomData<M>,
}

impl<A, M> BufLeave<A, M> {
    /// TODO: docs.
    pub fn buffer_id(mut self, buffer_id: BufferId) -> Self {
        self.buffer_id = Some(buffer_id);
        self
    }

    /// Creates a new [`BufLeave`] with the given action.
    pub fn new(action: A) -> Self {
        Self { action, buffer_id: None }
    }
}

impl<A, M> AutoCommand for BufLeave<A, M>
where
    A: for<'ctx> Action<M, Args = BufLeaveArgs, Ctx<'ctx> = NeovimCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    type Action = BufLeaveAction<A, M>;
    type OnModule = M;

    fn into_action(self) -> Self::Action {
        self.action
    }

    fn on_event(&self) -> AutoCommandEvent {
        AutoCommandEvent::BufLeave
    }

    fn on_buffer(&self) -> Option<BufferId> {
        self.buffer_id
    }

    fn take_actor_id(_: &AutoCommandCtx<'_>) -> ActorId {
        // TODO: Implement this.
        ActorId::unknown()
    }
}

impl<A, M> Action<M> for BufLeaveAction<A, M>
where
    A: for<'ctx> Action<M, Args = BufLeaveArgs, Ctx<'ctx> = NeovimCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    const NAME: ActionName = A::NAME;
    type Args = ActorId;
    type Ctx<'ctx> = &'ctx AutoCommandCtx<'ctx>;
    type Docs = A::Docs;
    type Return = A::Return;

    fn execute<'a>(
        &'a mut self,
        actor_id: Self::Args,
        ctx: Self::Ctx<'a>,
    ) -> impl MaybeResult<Self::Return> {
        let old_buffer_id = BufferId::new(ctx.args().buffer.clone());
        let args = BufLeaveArgs { actor_id, old_buffer_id };
        self.action.execute(args, ctx.deref().clone())
    }

    fn docs(&self) -> Self::Docs {
        self.action.docs()
    }
}
