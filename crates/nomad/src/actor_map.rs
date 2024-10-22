use nohash::IntMap as NoHashMap;

use crate::neovim::BufferId;
use crate::ActorId;

#[derive(Default)]
pub(crate) struct ActorMap {
    /// Map from [`BufferId`] to the [`ActorId`] that last added it.
    buffer_addition: NoHashMap<BufferId, ActorId>,

    /// Map from [`BufferId`] to the [`ActorId`] that last edited it.
    edit: NoHashMap<BufferId, ActorId>,
}

impl ActorMap {
    /// Registers the given [`ActorId`] as the last one to add the given
    /// buffer.
    pub(crate) fn added_buffer(
        &mut self,
        buffer_id: BufferId,
        actor_id: ActorId,
    ) {
        self.buffer_addition.insert(buffer_id, actor_id);
    }

    /// Registers the given [`ActorId`] as the last one to edit the given
    /// buffer.
    pub(crate) fn edited_buffer(
        &mut self,
        buffer_id: BufferId,
        actor_id: ActorId,
    ) {
        self.edit.insert(buffer_id, actor_id);
    }

    /// Removes the [`ActorId`] that last added the given buffer.
    pub(crate) fn take_added_buffer(
        &mut self,
        buffer_id: &BufferId,
    ) -> ActorId {
        self.buffer_addition.remove(buffer_id).unwrap_or(ActorId::unknown())
    }

    /// Removes the [`ActorId`] that last edited the given buffer.
    pub(crate) fn take_edited_buffer(
        &mut self,
        buffer_id: &BufferId,
    ) -> ActorId {
        self.edit.remove(buffer_id).unwrap_or(ActorId::unknown())
    }
}

// Storing a map from "action id" to ActorId is problematic. It works if
// there's a single autocommand on e.g. BufAdd. But if there's 2, the first one
// that executes in gonna take the actor id, and the second one will always get
// none.
//
// With the stream based approach we didn't have this problem because we'd only
// actually register the autocommand the first time an event was subscribed to.
// If the Context got an event which was already subscribed to, it would just
// return a copy of the `Subscription`.
//
// Do we want to re-use that model?
//
// That means there's no 1:1 relation between the user-visible autocommands
// stored either globally or on a buffer, and the corresponding autocommand
// stored on the Rust side.
//
// It also means that if the user removes one single autocommand, for whatever
// reason, everything stops working.
//
// However, for that we get more control and possibly faster execution, since
// we only have to transform the AutocmdCallBackArgs into the args for the
// given autocommand once, instead of once per autocommand.
//
// However, it also means that we have to re-implement more of the filtering
// logic. Like buffer-local autocommands, pattern autocmds,
//
// - pattern
// - buffer-local
//
// Maybe we just don't implement pattern.
//
// How would we instead implement it if we leave registering autocommands up to
// Neovim?
//
// There would have to be a way to reset the Shared<Option<ActorId>> we give to
// the body of every autocommand to None (btw, we weren't handling this in the
// old code), or it would still be set to the previous value the next time the
// autocmd fires. I don't think Neovim allows us to do that, so the first
// option is the only viable one.
