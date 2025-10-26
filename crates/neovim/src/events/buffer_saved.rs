use core::mem;

use editor::{AccessMut, AgentId, Editor};
use nohash::IntMap as NoHashMap;

use crate::Neovim;
use crate::buffer::{BufferExt, BufferId, NeovimBuffer};
use crate::events::{AutocmdId, Callbacks, Event, EventKind, Events};
use crate::oxi::api;
use crate::utils::CallbackExt;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BufferSaved(pub(crate) BufferId);

#[derive(Debug)]
pub(crate) struct BufferSavedRegisterOutput {
    autocmd_id: AutocmdId,
    saved_by: AgentId,
}

impl BufferSavedRegisterOutput {
    pub(crate) fn set_saved_by(&mut self, agent_id: AgentId) {
        self.saved_by = agent_id;
    }
}

impl Event for BufferSaved {
    type Args<'a> = (NeovimBuffer<'a>, AgentId);
    type Container<'ev> = &'ev mut NoHashMap<BufferId, Callbacks<Self>>;
    type RegisterOutput = BufferSavedRegisterOutput;

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        &mut events.on_buffer_saved
    }

    #[inline]
    fn key(&self) -> BufferId {
        self.0
    }

    #[inline]
    fn kind(&self) -> EventKind {
        EventKind::BufferSaved(*self)
    }

    #[inline]
    fn register(
        &self,
        events: &Events,
        mut nvim: impl AccessMut<Neovim> + 'static,
    ) -> Self::RegisterOutput {
        let callback = (move |args: api::types::AutocmdCallbackArgs| {
            nvim.with_mut(|nvim| {
                let buffer_id = BufferId::from(args.buffer.clone());

                let Some(mut buffer) = nvim.buffer(buffer_id) else {
                    tracing::error!(
                        buffer_name = %args.buffer.name(),
                        "BufWritePost triggered for an invalid buffer",
                    );
                    return true;
                };

                let Some(callbacks) =
                    buffer.nvim.events.on_buffer_saved.get_mut(&buffer_id)
                else {
                    return true;
                };

                let saved_by =
                    mem::take(&mut callbacks.register_output_mut().saved_by);

                for callback in callbacks.cloned() {
                    callback((buffer.reborrow(), saved_by));
                }

                false
            })
        })
        .catch_unwind()
        .map(|maybe_detach| maybe_detach.unwrap_or(true))
        .into_function();

        let autocmd_id = api::create_autocmd(
            ["BufWritePost"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .buffer(self.0.into())
                .callback(callback)
                .build(),
        )
        .expect("couldn't create autocmd on BufWritePost");

        BufferSavedRegisterOutput { autocmd_id, saved_by: AgentId::UNKNOWN }
    }

    #[inline]
    fn unregister(output: Self::RegisterOutput) {
        let _ = api::del_autocmd(output.autocmd_id);
    }
}
