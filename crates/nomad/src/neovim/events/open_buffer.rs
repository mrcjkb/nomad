use core::cmp::Ordering;

use nvim_oxi::api;

use crate::neovim::{BufferId, Neovim};
use crate::{ActorId, Context, Emitter, Event, Shared};

/// TODO: docs.
pub struct OpenBuffer {
    id: BufferId,
    opened_by: ActorId,
}

/// TODO: docs.
pub struct OpenBufferEvent {
    send_current: bool,
    next_buffer_opened_by: Shared<Option<ActorId>>,
}

impl OpenBuffer {
    /// TODO: docs.
    pub fn id(&self) -> BufferId {
        self.id.clone()
    }

    /// TODO: docs.
    pub fn opened_by(&self) -> ActorId {
        self.opened_by
    }
}

impl PartialEq for OpenBufferEvent {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for OpenBufferEvent {}

impl PartialOrd for OpenBufferEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenBufferEvent {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Event<Neovim> for OpenBufferEvent {
    type Payload = OpenBuffer;
    type SubscribeCtx = u32;

    fn subscribe(
        &mut self,
        emitter: Emitter<Self::Payload>,
        _: &Context<Neovim>,
    ) -> Self::SubscribeCtx {
        if self.send_current {
            for id in api::list_bufs().map(BufferId::new) {
                if id.is_of_text_buffer() {
                    let opened_by = self
                        .next_buffer_opened_by
                        .with_mut(Option::take)
                        .unwrap_or(ActorId::unknown());
                    emitter.send(OpenBuffer { opened_by, id });
                }
            }
        }

        let opts = api::opts::CreateAutocmdOpts::builder()
            .callback({
                let next_buffer_opened_by = self.next_buffer_opened_by.clone();
                move |args: api::types::AutocmdCallbackArgs| {
                    let id = BufferId::new(args.buffer);

                    if id.is_of_text_buffer() {
                        let opened_by = next_buffer_opened_by
                            .with_mut(Option::take)
                            .unwrap_or(ActorId::unknown());
                        emitter.send(OpenBuffer { opened_by, id });
                    }

                    false
                }
            })
            .build();

        api::create_autocmd(["BufAdd"], &opts)
            .expect("all arguments are valid")
    }

    fn unsubscribe(&mut self, id: u32, _: &Context<Neovim>) {
        let _ = api::del_autocmd(id);
    }
}
