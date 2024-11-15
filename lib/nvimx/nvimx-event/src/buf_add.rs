use core::marker::PhantomData;
use core::ops::Deref;

use nvimx_action::{Action, ActionName, IntoModuleName};
use nvimx_common::MaybeResult;
use nvimx_ctx::{
    ActorId,
    AutoCommand,
    AutoCommandCtx,
    AutoCommandEvent,
    BufferCtx,
    BufferId,
    ShouldDetach,
};

/// TODO: docs.
pub struct BufAdd<A, M> {
    action: BufAddAction<A, M>,
    buffer_id: Option<BufferId>,
}

pub struct BufAddAction<A, M> {
    action: A,
    module_name: PhantomData<M>,
}

impl<A, M> BufAdd<A, M> {
    /// TODO: docs.
    pub fn buffer_id(mut self, buffer_id: BufferId) -> Self {
        self.buffer_id = Some(buffer_id);
        self
    }

    /// Creates a new [`BufAdd`] with the given action.
    pub fn new(action: A) -> Self {
        Self {
            action: BufAddAction { action, module_name: PhantomData },
            buffer_id: None,
        }
    }
}

impl<A, M> AutoCommand for BufAdd<A, M>
where
    A: for<'ctx> Action<M, Args = ActorId, Ctx<'ctx> = BufferCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    type Action = BufAddAction<A, M>;
    type OnModule = M;

    fn into_action(self) -> Self::Action {
        self.action
    }

    fn on_event(&self) -> AutoCommandEvent {
        AutoCommandEvent::BufAdd
    }

    fn on_buffer(&self) -> Option<BufferId> {
        self.buffer_id
    }

    fn take_actor_id(ctx: &AutoCommandCtx<'_>) -> ActorId {
        let buffer_id = BufferId::new(ctx.args().buffer.clone());
        ctx.with_actor_map(|m| m.take_added_buffer(&buffer_id))
    }
}

impl<A, M> Action<M> for BufAddAction<A, M>
where
    A: for<'ctx> Action<M, Args = ActorId, Ctx<'ctx> = BufferCtx<'ctx>>,
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
        actor_id: ActorId,
        ctx: Self::Ctx<'a>,
    ) -> impl MaybeResult<Self::Return> {
        let buffer_id = BufferId::new(ctx.args().buffer.clone());
        let buffer_ctx = ctx
            .deref()
            .clone()
            .into_buffer(buffer_id)
            .expect("buffer was just added, so its ID must be valid");
        self.action.execute(actor_id, buffer_ctx)
    }

    fn docs(&self) -> Self::Docs {
        self.action.docs()
    }
}
