use nomad::autocmds::BufUnloadArgs;
use nomad::{action_name, Action, ActionName, Shared};
use nomad_server::Message;

use super::SessionCtx;
use crate::Collab;

pub(super) struct DetachBufferActions {
    pub(super) message_tx: flume::Sender<Message>,
    pub(super) session_ctx: Shared<SessionCtx>,
}

impl Action for DetachBufferActions {
    const NAME: ActionName = action_name!("detach-buffer-actions");
    type Args = BufUnloadArgs;
    type Docs = ();
    type Module = Collab;
    type Return = ();

    fn execute(&mut self, _args: Self::Args) {
        todo!();
    }

    fn docs(&self) {}
}
