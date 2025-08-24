use editor::{AccessMut, AgentId, Editor, Shared};

use crate::Neovim;
use crate::buffer::{BufferId, NeovimBuffer};
use crate::events::{AutocmdId, Callbacks, Event, EventKind, Events};
use crate::oxi::api;
use crate::utils::CallbackExt;

#[derive(Clone, Copy)]
pub(crate) struct BufferCreated;

impl Event for BufferCreated {
    type Args<'a> = (NeovimBuffer<'a>, AgentId);
    type Container<'ev> = &'ev mut Option<Callbacks<Self>>;
    type RegisterOutput = (AutocmdId, AutocmdId);

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        &mut events.on_buffer_created
    }

    #[inline]
    fn key(&self) {}

    #[inline]
    fn kind(&self) -> EventKind {
        EventKind::BufferCreated(*self)
    }

    #[inline]
    fn register(
        &self,
        events: &Events,
        mut nvim: impl AccessMut<Neovim> + 'static,
    ) -> Self::RegisterOutput {
        let old_name_was_empty = Shared::<bool>::new(false);

        let callback = {
            let old_name_was_empty = old_name_was_empty.clone();
            move |args: api::types::AutocmdCallbackArgs| {
                old_name_was_empty.set(
                    args.buffer
                        .get_name()
                        .expect("failed to get buffer name")
                        .is_empty(),
                );
                false
            }
        }
        .into_function();

        let autocmd_id_1 = api::create_autocmd(
            ["BufFilePre"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .callback(callback)
                .build(),
        )
        .expect("couldn't create autocmd");

        let callback = (move |args: api::types::AutocmdCallbackArgs| {
            nvim.with_mut(|nvim| {
                let buffer_id = BufferId::from(args.buffer.clone());

                let Some(mut buffer) = nvim.buffer(buffer_id) else {
                    return false;
                };

                // We should only treat buffer renames as creations if the old
                // name is empty. Renames from non-empty names should be
                // skipped.
                if args.event == "BufFilePost" && !old_name_was_empty.take() {
                    return false;
                }

                let events = &mut buffer.nvim.events;

                let Some(callbacks) = &events.on_buffer_created else {
                    return true;
                };

                let created_by = events
                    .agent_ids
                    .created_buffer
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

        let autocmd_id_2 = api::create_autocmd(
            ["BufReadPost", "BufNewFile", "BufFilePost"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .callback(callback)
                .build(),
        )
        .expect("couldn't create autocmd");

        (autocmd_id_1, autocmd_id_2)
    }

    #[inline]
    fn unregister((autocmd_id_1, autocmd_id_2): Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id_1);
        let _ = api::del_autocmd(autocmd_id_2);
    }
}
