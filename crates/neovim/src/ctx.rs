use core::mem;
use core::ops::Deref;

use pond::Engine;

use crate::{Get, Set};

/// TODO: docs
pub(crate) struct Ctx {
    engine: Engine,
}

/// TODO: docs
pub struct InitCtx {
    ctx: Ctx,
}

impl InitCtx {
    /// TODO: docs
    #[inline]
    pub fn new_input<T>(&self, input: T) -> (Get<T>, Set<T>) {
        let (get, set) = self.ctx.engine.var(input);
        (Get::new(get), Set::new(set))
    }
}

/// TODO: docs
pub struct GetCtx {
    ctx: Ctx,
}

/// TODO: docs
pub struct SetCtx {
    ctx: Ctx,
}

impl Deref for SetCtx {
    type Target = GetCtx;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `SetCtx` and `GetCtx` have the same layout.
        unsafe { mem::transmute(&self.ctx) }
    }
}
