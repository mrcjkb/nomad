use nomad::autocmds::BufAddArgs;
use nomad::{action_name, Action, ActionName, Shared};
use nomad_server::Message;

use super::SessionCtx;
use crate::Collab;

pub(super) struct RegisterBufferActions {
    message_tx: flume::Sender<Message>,
    session_ctx: Shared<SessionCtx>,
}

impl RegisterBufferActions {
    pub(super) fn new(
        message_tx: flume::Sender<Message>,
        session_ctx: Shared<SessionCtx>,
    ) -> Self {
        Self { message_tx, session_ctx }
    }
}

impl Action for RegisterBufferActions {
    const NAME: ActionName = action_name!("register-buffer-actions");
    type Args = BufAddArgs;
    type Docs = ();
    type Module = Collab;
    type Return = ();

    fn execute(&mut self, _args: Self::Args) {
        todo!();
    }

    fn docs(&self) {}
}
