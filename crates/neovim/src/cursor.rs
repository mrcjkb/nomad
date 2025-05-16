//! TODO: docs.

use ed::ByteOffset;
use ed::backend::{AgentId, Buffer, Cursor};

use crate::Neovim;
use crate::buffer::{BufferId, NeovimBuffer, Point};
use crate::events::{self, EventHandle, Events};
use crate::oxi::api;

/// TODO: docs.
#[derive(Copy, Clone)]
pub struct NeovimCursor<'a> {
    buffer: NeovimBuffer<'a>,
}

impl<'a> NeovimCursor<'a> {
    /// TODO: docs.
    #[inline]
    pub(crate) fn new(buffer: NeovimBuffer<'a>) -> Self {
        debug_assert!(buffer.is_focused());
        Self { buffer }
    }
}

impl Cursor for NeovimCursor<'_> {
    type Backend = Neovim;

    #[inline]
    fn buffer_id(&self) -> BufferId {
        self.buffer.id()
    }

    #[inline]
    fn byte_offset(&self) -> ByteOffset {
        let (row, col) =
            api::Window::current().get_cursor().expect("couldn't get cursor");

        self.buffer.byte_offset_of_point(Point {
            line_idx: row - 1,
            byte_offset: col.into(),
        })
    }

    #[inline]
    fn id(&self) -> BufferId {
        self.buffer.id()
    }

    #[inline]
    fn r#move(&mut self, offset: ByteOffset, _agent_id: AgentId) {
        let point = self.buffer.point_of_byte_offset(offset);

        api::Window::current()
            .set_cursor(point.line_idx + 1, point.byte_offset.into())
            .expect("couldn't set cursor");
    }

    #[inline]
    fn on_moved<Fun>(&self, mut fun: Fun) -> EventHandle
    where
        Fun: FnMut(&NeovimCursor<'_>, AgentId) + 'static,
    {
        Events::insert(
            self.buffer.events().clone(),
            events::CursorMoved(self.buffer.id()),
            move |(this, moved_by)| fun(this, moved_by),
        )
    }

    #[inline]
    fn on_removed<Fun>(&self, mut fun: Fun) -> EventHandle
    where
        Fun: FnMut(&NeovimCursor<'_>, AgentId) + 'static,
    {
        Events::insert(
            self.buffer.events().clone(),
            events::BufLeave(self.buffer.id()),
            move |(&buf, unfocused_by)| {
                fun(&NeovimCursor::new(buf), unfocused_by)
            },
        )
    }
}
