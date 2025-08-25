use editor::{AccessMut, AgentId, Shared};
use nohash::IntMap as NoHashMap;

use crate::Neovim;
use crate::buffer::BufferId;
use crate::events::{AutocmdId, Callbacks, Event, EventKind, Events};
use crate::oxi::api;
use crate::utils::CallbackExt;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BufferRemoved(pub(crate) BufferId);

impl Event for BufferRemoved {
    type Args<'a> = (BufferId, AgentId);
    type Container<'ev> = &'ev mut NoHashMap<BufferId, Callbacks<Self>>;
    type RegisterOutput = (AutocmdId, AutocmdId);

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        &mut events.on_buffer_removed
    }

    #[inline]
    fn key(&self) -> BufferId {
        self.0
    }

    #[inline]
    fn kind(&self) -> EventKind {
        EventKind::BufferRemoved(*self)
    }

    #[allow(clippy::too_many_lines)]
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
                .buffer(self.0.into())
                .callback(callback)
                .build(),
        )
        .expect("couldn't create autocmd");

        let callback = (move |args: api::types::AutocmdCallbackArgs| {
            // We should only treat buffer renames as removals if the old name
            // wasn't empty and the new name is.
            if args.event == "BufFilePost" {
                if old_name_was_empty.take() {
                    return false;
                }

                let new_name =
                    args.buffer.get_name().expect("failed to get buffer name");

                if !new_name.is_empty() {
                    return false;
                }
            }

            nvim.with_mut(|nvim| {
                let events = &mut nvim.events;

                let buffer_id = BufferId::from(args.buffer);

                let Some(callbacks) = events.on_buffer_removed.get(&buffer_id)
                else {
                    return true;
                };

                let removed_by = events
                    .agent_ids
                    .removed_buffer
                    .remove(&buffer_id)
                    .unwrap_or(AgentId::UNKNOWN);

                for callback in callbacks.cloned() {
                    callback((buffer_id, removed_by));
                }

                false
            })
        })
        .catch_unwind()
        .map(|maybe_detach| maybe_detach.unwrap_or(true))
        .into_function();

        let autocmd_id_2 = api::create_autocmd(
            ["BufUnload", "BufFilePost"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .buffer(self.0.into())
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
