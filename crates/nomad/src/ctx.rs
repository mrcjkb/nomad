//! TODO: docs.

use nvim_oxi::api::types;

use crate::actor_map::ActorMap;
use crate::autocmd::{AugroupId, AutoCommandEvent};
use crate::neovim::BufferId;
use crate::{Boo, Shared};

/// TODO: docs.
pub struct AutoCommandCtx<'ctx> {
    args: types::AutocmdCallbackArgs,
    event: AutoCommandEvent,
    ctx: Boo<'ctx, Ctx>,
}

/// TODO: docs.
pub struct NeovimCtx<'ctx> {
    ctx: Boo<'ctx, Ctx>,
}

/// TODO: docs.
pub struct BufferCtx<'ctx> {
    buffer_id: BufferId,
    ctx: Boo<'ctx, Ctx>,
}

/// TODO: docs.
pub struct FileCtx<'ctx> {
    ctx: BufferCtx<'ctx>,
}

/// TODO: docs.
pub struct TextBufferCtx<'ctx> {
    ctx: BufferCtx<'ctx>,
}

/// TODO: docs.
pub struct TextFileCtx<'ctx> {
    ctx: BufferCtx<'ctx>,
}

impl<'ctx> AutoCommandCtx<'ctx> {
    /// Returns a shared reference to the autocmd's args.
    pub fn args(&self) -> &types::AutocmdCallbackArgs {
        &self.args
    }

    pub fn as_ref(&self) -> AutoCommandCtx<'_> {
        AutoCommandCtx {
            args: self.args.clone(),
            event: self.event,
            ctx: self.ctx.clone(),
        }
    }

    /// Consumes `self` and returns the arguments passed to the autocmd.
    pub fn into_args(self) -> types::AutocmdCallbackArgs {
        self.args
    }

    pub fn with_actor_map<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut ActorMap) -> R,
    {
        self.ctx.with_inner(|inner| fun(&mut inner.actor_map))
    }

    pub(crate) fn new(
        args: types::AutocmdCallbackArgs,
        event: AutoCommandEvent,
        neovim_ctx: NeovimCtx<'ctx>,
    ) -> Self {
        Self { args, event, ctx: neovim_ctx.ctx }
    }
}

impl NeovimCtx<'_> {
    pub(crate) fn augroup_id(&self) -> AugroupId {
        todo!();
    }

    pub(crate) fn as_ref(&self) -> NeovimCtx<'_> {
        NeovimCtx { ctx: self.ctx.as_ref() }
    }

    pub(crate) fn to_static(&self) -> NeovimCtx<'static> {
        NeovimCtx { ctx: self.ctx.clone().into_owned() }
    }
}

#[derive(Default, Clone)]
pub(crate) struct Ctx {
    inner: Shared<CtxInner>,
}

impl Ctx {
    fn with_inner<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut CtxInner) -> R,
    {
        self.inner.with_mut(|inner| fun(inner))
    }
}

#[derive(Default)]
struct CtxInner {
    actor_map: ActorMap,
}

impl Clone for NeovimCtx<'_> {
    fn clone(&self) -> Self {
        Self { ctx: self.ctx.clone() }
    }
}
