use collab_server::message::Peer;
use nvimx::ByteOffset;
use nvimx::ctx::{BufferCtx, BufferId, Selection};
use nvimx::diagnostics::HighlightGroup;

/// TODO: docs.
pub(super) struct PeerTooltip {
    /// TUIs can only display a single line cursor at a time, so we select the
    /// character immediately to the right of the cursor offset to fake a block
    /// cursor.
    selection: Selection,
    in_buffer: BufferId,
    peer: Peer,
}

impl PeerTooltip {
    /// The [`BufferId`] this tooltip is in.
    pub(super) fn buffer_id(&self) -> BufferId {
        self.in_buffer
    }

    pub(super) fn create(
        peer: Peer,
        byte_offset: ByteOffset,
        ctx: BufferCtx<'_>,
    ) -> Self {
        let hl_group = HighlightGroup::special();
        let byte_range = byte_offset..(byte_offset + 1).min(ctx.byte_len());
        Self {
            selection: ctx.create_selection(byte_range, hl_group),
            in_buffer: ctx.buffer_id(),
            peer,
        }
    }

    pub(super) fn relocate(&mut self, new_offset: ByteOffset) {
        // TODO: bound the offset to the buffer length.
        // TODO: does Neovim already snap the end of the selection to the
        // correct char boundary if it falls within a multi-byte character?
        self.selection.set_byte_range(new_offset..(new_offset + 1));
    }
}
