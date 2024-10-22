//! TODO: docs.

use nvim_oxi::api::types;

use crate::action_map::ActionMap;
use crate::{Boo, Shared};

/// TODO: docs.
pub struct AutocmdCtx<'ctx> {
    args: types::AutocmdCallbackArgs,
    ctx: Boo<'ctx, Ctx>,
}

/// TODO: docs.
pub struct NeovimCtx<'ctx> {
    ctx: Boo<'ctx, Ctx>,
}

impl<'ctx> AutocmdCtx<'ctx> {
    /// Returns a shared reference to the autocmd's args.
    pub fn args(&self) -> &types::AutocmdCallbackArgs {
        &self.args
    }

    /// Consumes `self` and returns the arguments passed to the autocmd.
    pub fn into_args(self) -> types::AutocmdCallbackArgs {
        self.args
    }

    pub fn with_action_map<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut ActionMap) -> R,
    {
        self.ctx.with_inner(|inner| fun(&mut inner.action_map))
    }

    pub(crate) fn new(
        args: types::AutocmdCallbackArgs,
        neovim_ctx: NeovimCtx<'ctx>,
    ) -> Self {
        Self { args, ctx: neovim_ctx.ctx }
    }
}

#[derive(Default)]
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
    action_map: ActionMap,
}
