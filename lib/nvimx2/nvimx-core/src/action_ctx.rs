use core::ops::{Deref, DerefMut};

use smallvec::{SmallVec, smallvec};

use crate::backend::BackendExt;
use crate::notify::{Emitter, Source};
use crate::{Backend, Name, NeovimCtx, Plugin, notify};

/// TODO: docs.
pub struct ActionCtx<'a, P, B> {
    neovim_ctx: NeovimCtx<'a, P, B>,
    module_path: &'a ModulePath,
    action_name: Name,
}

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    names: SmallVec<[Name; 2]>,
}

impl<'a, P, B> ActionCtx<'a, P, B>
where
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn emit_info(&mut self, message: notify::Message) {
        self.neovim_ctx.backend_mut().emitter().emit(notify::Notification {
            level: notify::Level::Info,
            source: Source {
                module_path: &self.module_path,
                action_name: Some(self.action_name),
            },
            message,
            updates_prev: None,
        });
    }

    #[inline]
    pub(crate) fn emit_err<Err>(&mut self, err: Err)
    where
        Err: notify::Error<B>,
    {
        self.neovim_ctx.backend_mut().emit_err::<P, _>(
            Source {
                module_path: &self.module_path,
                action_name: Some(self.action_name),
            },
            err,
        );
    }

    #[inline]
    pub(crate) fn module_path(&self) -> &'a ModulePath {
        self.module_path
    }

    /// Constructs a new [`ActionCtx`].
    #[inline]
    pub(crate) fn new(
        neovim_ctx: NeovimCtx<'a, P, B>,
        module_path: &'a ModulePath,
        action_name: Name,
    ) -> Self {
        Self { neovim_ctx, module_path, action_name }
    }
}

impl ModulePath {
    /// TODO: docs.
    #[inline]
    pub fn names(&self) -> impl ExactSizeIterator<Item = Name> + '_ {
        self.names.iter().copied()
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(base_module: Name) -> Self {
        Self { names: smallvec![base_module] }
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn push(&mut self, module_name: Name) {
        self.names.push(module_name);
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn pop(&mut self) {
        self.names.pop();
    }
}

impl<'a, P, B> Deref for ActionCtx<'a, P, B> {
    type Target = NeovimCtx<'a, P, B>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.neovim_ctx
    }
}

impl<P, B> DerefMut for ActionCtx<'_, P, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.neovim_ctx
    }
}
