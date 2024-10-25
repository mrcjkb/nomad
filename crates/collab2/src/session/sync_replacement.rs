use nomad::buf_attach::BufAttachArgs;
use nomad::{action_name, Action, ActionName, Shared, ShouldDetach};
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

            // 1: get the `FileId` associated with the `BufferId`;
            // 2: get the corresponding `FileRefMut`;
            // 3: synchronize the replacement -> get the `Message`;
            // 4: for all windows displaying the buffer, update tooltips of all
            //    remote cursors and selections on the buffer;
            // 5: return `Message`
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
