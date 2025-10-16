use core::cell::Cell;

use collab_types::{Peer, PeerHandle};
use editor::ByteOffset;
use neovim::buffer::{BufferExt, Point};
use neovim::oxi::api;

use crate::editors::neovim::PeerHighlightGroup;

/// A remote peer's handle in a buffer, displayed either directly above or
/// below their [`cursor`](crate::editors::neovim::PeerCursor).
pub struct NeovimPeerHandle {
    /// The buffer the cursor is in.
    buffer: api::Buffer,

    /// The ID of the extmark used to display the handle.
    extmark_id: u32,

    /// The ID of the highlight group used to highlight the handle.
    hl_group_id: u32,

    /// The ID of the namespace the [`extmark_id`](Self::extmark_id) belongs
    /// to.
    namespace_id: u32,

    /// The remote peer's handle.
    peer_handle: PeerHandle,
}

/// The highlight group used to highlight a remote peer's handle.
pub(super) struct PeerHandleHighlightGroup;

impl PeerHandleHighlightGroup {
    thread_local! {
        static GROUP_IDS: Cell<[u32; 16]> = const { Cell::new([0; _]) };
    }
}

impl NeovimPeerHandle {
    /// Creates a new handle for the given remote peer to be displayed above or
    /// below the cursor at the given offset in the given buffer.
    pub(super) fn create(
        peer: Peer,
        mut buffer: api::Buffer,
        cursor_offset: ByteOffset,
        namespace_id: u32,
    ) -> Self {
        let hl_group_id = PeerHandleHighlightGroup::group_id(peer.id);

        let (line, col, mut opts_builder) = Self::extmark_params(
            buffer.clone(),
            cursor_offset,
            &peer.handle,
            hl_group_id,
        );

        let extmark_id = buffer
            .set_extmark(namespace_id, line, col, &opts_builder.build())
            .expect("couldn't create extmark");

        Self {
            buffer,
            extmark_id,
            hl_group_id,
            namespace_id,
            peer_handle: peer.handle,
        }
    }

    /// Moves the handle to keep it in sync with the new cursor offset.
    pub(super) fn r#move(&mut self, new_cursor_offset: ByteOffset) {
        let (line, col, mut opts_builder) = Self::extmark_params(
            self.buffer.clone(),
            new_cursor_offset,
            &self.peer_handle,
            self.hl_group_id,
        );

        let opts = opts_builder.id(self.extmark_id).build();

        let new_extmark_id = self
            .buffer
            .set_extmark(self.namespace_id, line, col, &opts)
            .expect("couldn't move extmark");

        debug_assert_eq!(new_extmark_id, self.extmark_id);
    }

    /// Removes the handle from the buffer.
    pub(super) fn remove(mut self) {
        self.buffer
            .del_extmark(self.namespace_id, self.extmark_id)
            .expect("couldn't delete extmark");
    }

    /// Returns the line, column, and options to give to
    /// [`api::Buffer::set_extmark`] to position the peer handle above or below
    /// the cursor at the given byte offset.
    fn extmark_params(
        buffer: api::Buffer,
        cursor_offset: ByteOffset,
        _peer_handle: &PeerHandle,
        _hl_group_id: u32,
    ) -> (usize, usize, api::opts::SetExtmarkOptsBuilder) {
        todo!();
    }
}

impl PeerHighlightGroup for PeerHandleHighlightGroup {
    const NAME_PREFIX: &str = "NomadCollabPeerHandle";

    fn set_hl_opts() -> api::opts::SetHighlightOpts {
        api::opts::SetHighlightOpts::builder().link("PmenuSel").build()
    }

    fn with_group_ids<R>(fun: impl FnOnce(&[Cell<u32>]) -> R) -> R {
        Self::GROUP_IDS.with(|ids| fun(ids.as_array_of_cells().as_slice()))
    }
}
