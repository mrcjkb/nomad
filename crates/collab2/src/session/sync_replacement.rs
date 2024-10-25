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

    fn execute(&mut self, _args: Self::Args) {
        todo!();
    }

    fn docs(&self) {}
}
