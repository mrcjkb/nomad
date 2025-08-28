use editor::{AccessMut, AgentId, Editor, Shared};

use crate::Neovim;
use crate::buffer::{BufferExt, BufferId, NeovimBuffer};
use crate::events::{AutocmdId, Callbacks, Event, EventKind, Events};
use crate::oxi::api;
use crate::utils::CallbackExt;

#[derive(Debug, Clone, Copy)]
pub(crate) struct CursorCreated;

impl Event for CursorCreated {
    type Args<'a> = (NeovimBuffer<'a>, AgentId);
    type Container<'ev> = &'ev mut Option<Callbacks<Self>>;
    type RegisterOutput = AutocmdId;

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        &mut events.on_cursor_created
    }

    #[inline]
    fn kind(&self) -> EventKind {
        EventKind::CursorCreated(*self)
    }

    #[inline]
    fn key(&self) {}

    #[inline]
    fn register(
        &self,
        events: &Events,
        mut nvim: impl AccessMut<Neovim> + 'static,
    ) -> Self::RegisterOutput {
        let current_buffer = api::Buffer::current();

        let old_buffer_was_unnamed =
            Shared::<bool>::new(current_buffer.name().is_empty());

        let old_buffer_id =
            Shared::<BufferId>::new(BufferId::from(current_buffer));

        let on_buf_enter = (move |args: api::types::AutocmdCallbackArgs| {
            let old_buffer_was_unnamed =
                old_buffer_was_unnamed.replace(args.buffer.name().is_empty());

            let buffer_id = BufferId::from(args.buffer);

            let old_buffer_id = old_buffer_id.replace(buffer_id);

            // Some commands like ":edit" or ":split" can cause BufEnter to be
            // fired multiple times for the same buffer, so we need to make
            // sure that the buffer has actually changed.
            //
            // The only exception can happen when the user ":edit"s a new file
            // while in an unnamed buffer, in which case the file's contents
            // will be loaded into the current buffer without the bufnr
            // changing.
            if old_buffer_id == buffer_id && !old_buffer_was_unnamed {
                return false;
            }

            nvim.with_mut(|nvim| {
                let Some(mut buffer) = nvim.buffer(buffer_id) else {
                    return false;
                };

                let events = &mut buffer.nvim.events;

                let Some(callbacks) = &events.on_cursor_created else {
                    return true;
                };

                let created_by = events
                    .agent_ids
                    .created_cursor
                    .remove(&buffer_id)
                    .unwrap_or(AgentId::UNKNOWN);

                for callback in callbacks.cloned() {
                    callback((buffer.reborrow(), created_by));
                }

                false
            })
        })
        .catch_unwind()
        .map(|maybe_detach| maybe_detach.unwrap_or(true))
        .into_function();

        api::create_autocmd(
            ["BufEnter"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .callback(on_buf_enter)
                .build(),
        )
        .expect("couldn't create autocmd on BufEnter")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }
}
