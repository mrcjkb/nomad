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

    fn execute(&mut self, _args: Self::Args) {
        todo!();
    }

    fn docs(&self) {}
}
