//! TODO: docs.

use core::ops::Range;

use editor::{AccessMut, AgentId, Buffer, ByteOffset, Selection, Shared};
use nvim_oxi::api;

use crate::buffer::{BufferId, NeovimBuffer};
use crate::buffer_ext::BufferExt;
use crate::events::EventHandle;
use crate::{Neovim, events};

/// TODO: docs.
pub struct NeovimSelection<'a> {
    /// The buffer the selection is in.
    buffer: api::Buffer,

    /// An exclusive reference to the Neovim instance.
    pub(crate) nvim: &'a mut Neovim,
}

impl<'a> From<NeovimBuffer<'a>> for NeovimSelection<'a> {
    #[inline]
    fn from(buffer: NeovimBuffer<'a>) -> Self {
        debug_assert!(buffer.is_focused());
        Self { buffer: buffer.clone(), nvim: buffer.nvim }
    }
}

impl Selection for NeovimSelection<'_> {
    type Editor = Neovim;

    #[inline]
    fn buffer_id(&self) -> BufferId {
        self.buffer.clone().into()
    }

    #[inline]
    fn byte_range(&self) -> Range<ByteOffset> {
        self.buffer.selection().expect("buffer has a selection")
    }

    #[inline]
    fn id(&self) -> BufferId {
        self.buffer_id()
    }

    #[inline]
    fn on_moved<Fun>(
        &mut self,
        fun: Fun,
        nvim: impl AccessMut<Self::Editor> + Clone + 'static,
    ) -> EventHandle
    where
        Fun: FnMut(&NeovimSelection, AgentId) + 'static,
    {
        let is_selection_alive = Shared::<bool>::new(true);
        let fun = Shared::<Fun>::new(fun);

        let buffer_id = self.buffer_id();

        let cursor_moved_handle = self.nvim.events.insert(
            events::CursorMoved(buffer_id),
            {
                let is_selection_alive = is_selection_alive.clone();
                let fun = fun.clone();
                move |(cursor, moved_by)| {
                    // Make sure the selection is still alive before calling
                    // the user's function.
                    if is_selection_alive.copied() {
                        let this = NeovimSelection {
                            buffer: cursor.buffer(),
                            nvim: cursor.nvim,
                        };
                        fun.with_mut(|fun| fun(&this, moved_by));
                    }
                }
            },
            nvim.clone(),
        );

        let mode_changed_handle = self.nvim.events.insert(
            events::ModeChanged,
            move |(buf, _old_mode, new_mode, changed_by)| {
                if buf.id() != buffer_id || !is_selection_alive.copied() {
                    return;
                }

                if new_mode.has_selected_range() {
                    let this = NeovimSelection::from(buf);
                    fun.with_mut(|fun| fun(&this, changed_by));
                } else {
                    is_selection_alive.set(false);
                }
            },
            nvim,
        );

        cursor_moved_handle.merge(mode_changed_handle)
    }

    #[inline]
    fn on_removed<Fun>(
        &mut self,
        mut fun: Fun,
        nvim: impl AccessMut<Self::Editor> + Clone + 'static,
    ) -> EventHandle
    where
        Fun: FnMut(BufferId, AgentId) + 'static,
    {
        let buffer_id = self.buffer_id();

        self.nvim.events.insert(
            events::ModeChanged,
            move |(buf, old_mode, new_mode, changed_by)| {
                if buf.id() == buffer_id
                    && old_mode.has_selected_range()
                    // A selection is only removed if the new mode isn't also
                    // displaying a selected range.
                    && !new_mode.has_selected_range()
                {
                    fun(buffer_id, changed_by);
                }
            },
            nvim,
        )
    }
}
