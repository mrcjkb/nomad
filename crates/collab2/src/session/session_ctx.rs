use e31e::fs::AbsPathBuf;
use nohash::IntMap as NoHashMap;
use nomad::{Action, ActorId, BufferId, Event, Shared, ShouldDetach};

#[derive(Clone)]
pub(super) struct SessionCtx {
    /// The [`ActorId`] of the [`Session`].
    pub(super) actor_id: ActorId,

    /// The absolute path to the root of the project.
    pub(super) project_root: AbsPathBuf,

    /// Map from [`BufferId`]
    pub(super) buffer_actions: NoHashMap<BufferId, Shared<ShouldDetach>>,
}
