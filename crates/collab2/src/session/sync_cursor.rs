use nomad::events::Cursor;
use nomad::{action_name, Action, ActionName, Shared, ShouldDetach};
use nomad_server::Message;

use super::SessionCtx;
use crate::Collab;

#[derive(Clone)]
pub(super) struct SyncCursor {
    pub(super) message_tx: flume::Sender<Message>,
    pub(super) session_ctx: Shared<SessionCtx>,
    pub(super) should_detach: Shared<ShouldDetach>,
}

impl Action for SyncCursor {
    const NAME: ActionName = action_name!("synchronize-cursor");
    type Args = Cursor;
    type Docs = ();
    type Module = Collab;
    type Return = ShouldDetach;

    fn execute(&mut self, cursor: Self::Args) -> Self::Return {
        let message = self.session_ctx.with_mut(|session_ctx| {
            if cursor.moved_by() == session_ctx.actor_id {
                return None;
            }

            // 1: get the `CursorId` associated with the `BufferId`;
            // 2: get the corresponding `CursorRefMut`;
            // 3: synchronize either its movement, creation, or deletion;
            // 4: return `Message`
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
