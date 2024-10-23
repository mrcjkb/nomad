use crate::neovim::BufferId;
use crate::{ActorId, Replacement};

/// TODO: docs.
pub struct BufAttachArgs {
    /// TODO: docs.
    pub actor_id: ActorId,

    /// TODO: docs.
    pub buffer_id: BufferId,

    /// TODO: docs.
    pub replacement: Replacement,
}
