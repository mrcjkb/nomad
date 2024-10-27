use core::ops::Range;

use nomad::ctx::BufferCtx;
use nomad::{BufferId, ByteOffset};

/// TODO: docs.
pub(super) struct PeerSelection {
    offset_range: Range<ByteOffset>,
    in_buffer: BufferId,
}

impl PeerSelection {
    /// The [`BufferId`] this tooltip is in.
    pub(super) fn buffer_id(&self) -> BufferId {
        self.in_buffer
    }

    pub(super) fn create(
        offset_range: Range<ByteOffset>,
        ctx: BufferCtx<'_>,
    ) -> Self {
        Self { offset_range, in_buffer: ctx.buffer_id() }
    }

    pub(super) fn relocate(&mut self, new_offset_range: Range<ByteOffset>) {
        if self.offset_range == new_offset_range {
            return;
        }
        todo!();
    }
}
