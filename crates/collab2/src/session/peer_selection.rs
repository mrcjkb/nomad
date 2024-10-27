use core::ops::Range;

use nomad::ctx::BufferCtx;
use nomad::diagnostics::HighlightGroup;
use nomad::{BufferId, ByteOffset, Selection};

/// TODO: docs.
pub(super) struct PeerSelection {
    selection: Selection,
    in_buffer: BufferId,
}

impl PeerSelection {
    /// The [`BufferId`] this tooltip is in.
    pub(super) fn buffer_id(&self) -> BufferId {
        self.in_buffer
    }

    pub(super) fn create(
        byte_range: Range<ByteOffset>,
        ctx: BufferCtx<'_>,
    ) -> Self {
        let hl_group = HighlightGroup::special();
        Self {
            selection: ctx.create_selection(byte_range, hl_group),
            in_buffer: ctx.buffer_id(),
        }
    }

    pub(super) fn relocate(&mut self, new_byte_range: Range<ByteOffset>) {
        self.selection.set_byte_range(new_byte_range);
    }
}
