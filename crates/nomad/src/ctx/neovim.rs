use core::future::Future;
use core::ops::{Deref, DerefMut};

use nvim_oxi::api;

use crate::actor_map::ActorMap;
use crate::autocmd::{AugroupId, AutoCommandMap};
use crate::buf_attach::BufAttachMap;
use crate::buffer_id::BufferId;
use crate::ctx::BufferCtx;
use crate::decoration_provider::{DecorationProvider, NamespaceId};
use crate::{ActorId, Boo, JoinHandle, Shared};

/// TODO: docs.
#[derive(Default, Clone)]
pub struct NeovimCtx<'ctx> {
    ctx: Boo<'ctx, Ctx>,
}

#[derive(Default, Clone)]
struct Ctx {
    inner: Shared<CtxInner>,
}

#[derive(Default)]
struct CtxInner {
    actor_map: ActorMap,
    augroup_id: NomadAugroupId,
    autocmd_map: AutoCommandMap,
    buf_attach_map: BufAttachMap,
    decoration_provider: Option<DecorationProvider>,
    namespace_id: NomadNamespaceId,
    next_actor_id: NomadActorId,
}

#[derive(Copy, Clone)]
struct NomadActorId(ActorId);

#[derive(Copy, Clone)]
struct NomadAugroupId(AugroupId);

#[derive(Copy, Clone)]
struct NomadNamespaceId(NamespaceId);

impl<'ctx> NeovimCtx<'ctx> {
    /// TODO: docs.
    pub fn into_buffer(self, buffer_id: BufferId) -> Option<BufferCtx<'ctx>> {
        BufferCtx::from_neovim(buffer_id, self)
    }

    /// TODO: docs.
    pub fn next_actor_id(&self) -> ActorId {
        self.ctx.with_inner(|inner| inner.next_actor_id.post_inc())
    }

    /// TODO: docs.
    pub fn reborrow(&self) -> NeovimCtx<'_> {
        NeovimCtx { ctx: self.ctx.as_ref() }
    }

    /// TODO: docs.
    pub fn spawn<F, Fut>(&self, callback: F) -> JoinHandle<Fut::Output>
    where
        F: FnOnce(NeovimCtx<'static>) -> Fut,
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        crate::executor::spawn(callback(self.to_static()))
    }

    /// TODO: docs.
    pub fn to_static(&self) -> NeovimCtx<'static> {
        NeovimCtx { ctx: self.ctx.clone().into_owned() }
    }

    pub(crate) fn augroup_id(&self) -> AugroupId {
        self.ctx.with_inner(|inner| inner.augroup_id.into())
    }

    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.ctx.with_inner(|inner| inner.namespace_id.into())
    }

    pub(crate) fn with_actor_map<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut ActorMap) -> R,
    {
        self.ctx.with_inner(|inner| fun(&mut inner.actor_map))
    }

    pub(crate) fn with_autocmd_map<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut AutoCommandMap) -> R,
    {
        self.ctx.with_inner(|inner| fun(&mut inner.autocmd_map))
    }

    pub(crate) fn with_buf_attach_map<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut BufAttachMap) -> R,
    {
        self.ctx.with_inner(|inner| fun(&mut inner.buf_attach_map))
    }

    pub(crate) fn with_decoration_provider<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut DecorationProvider) -> R,
    {
        self.ctx.with_inner(|inner| {
            let provider =
                inner.decoration_provider.get_or_insert_with(|| {
                    DecorationProvider::new(self.to_static())
                });
            fun(provider)
        })
    }
}

impl Ctx {
    fn with_inner<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut CtxInner) -> R,
    {
        self.inner.with_mut(|inner| fun(inner))
    }
}

impl Default for NomadActorId {
    fn default() -> Self {
        Self(ActorId::new(1))
    }
}

impl Default for NomadAugroupId {
    fn default() -> Self {
        let opts = api::opts::CreateAugroupOpts::builder().clear(true).build();
        let augroup_id =
            api::create_augroup(crate::Nomad::AUGROUP_NAME, &opts)
                .expect("all the arguments are valid")
                .into();
        Self(augroup_id)
    }
}

impl Default for NomadNamespaceId {
    fn default() -> Self {
        Self(NamespaceId::new(crate::Nomad::NAMESPACE_NAME))
    }
}

impl Deref for NomadActorId {
    type Target = ActorId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NomadActorId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<NomadAugroupId> for AugroupId {
    fn from(NomadAugroupId(id): NomadAugroupId) -> Self {
        id
    }
}

impl From<NomadNamespaceId> for NamespaceId {
    fn from(NomadNamespaceId(id): NomadNamespaceId) -> Self {
        id
    }
}
