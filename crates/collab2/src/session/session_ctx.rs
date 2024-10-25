use e31e::fs::AbsPathBuf;
use nohash::IntMap as NoHashMap;
use nomad::ctx::NeovimCtx;
use nomad::{ActorId, BufferId, Shared, ShouldDetach};

#[derive(Clone)]
pub(super) struct SessionCtx {
    /// The [`ActorId`] of the [`Session`].
    pub(super) actor_id: ActorId,

    /// The absolute path to the root of the project.
    pub(super) project_root: AbsPathBuf,

    /// Map from [`BufferId`]
    pub(super) buffer_actions: NoHashMap<BufferId, Shared<ShouldDetach>>,

    pub(super) neovim_ctx: NeovimCtx<'static>,
}
