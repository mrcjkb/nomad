use core::any::type_name;

use nomad::buf_attach::BufAttachArgs;
use nomad::{action_name, Action, ActionName, BufferId, Shared, ShouldDetach};
use nomad_server::Message;

use super::SessionCtx;
use crate::Collab;

pub(super) struct SyncReplacement {
    pub(super) message_tx: flume::Sender<Message>,
    pub(super) session_ctx: Shared<SessionCtx>,
    pub(super) should_detach: Shared<ShouldDetach>,
}

impl Action for SyncReplacement {
    const NAME: ActionName = action_name!("synchronize-replacement");
    type Args = BufAttachArgs;
    type Docs = ();
    type Module = Collab;
    type Return = ShouldDetach;

    fn execute(&mut self, args: Self::Args) -> Self::Return {
        let message = self.session_ctx.with_mut(|session_ctx| {
            if args.actor_id == session_ctx.actor_id {
                return None;
            }

            let Some(mut file) =
                session_ctx.file_mut_of_buffer_id(args.buffer_id)
            else {
                unreachable!(
                    "couldn't convert BufferId to file in {}",
                    type_name::<Self>()
                );
            };

            let edit = file.sync_edited_text([args.replacement.into()]);

            let file_id = file.id();
            session_ctx.refresh_cursors(file_id);
            session_ctx.refresh_selections(file_id);

            todo!();
        });

        if let Some(message) = message {
            if self.message_tx.send(message).is_err() {
                self.should_detach.set(ShouldDetach::Yes);
            }
        }

        self.should_detach.get()
    }

    fn docs(&self) {}
}
