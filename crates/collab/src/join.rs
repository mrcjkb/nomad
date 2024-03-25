use nomad::prelude::*;

use crate::{Collab, Config, SessionId};

pub(crate) struct Join {
    config: Get<Config>,
}

impl Join {
    pub(crate) fn new(config: Get<Config>) -> Self {
        Self { config }
    }
}

impl Action<Collab> for Join {
    const NAME: ActionName = action_name!("join");

    type Args = SessionId;

    type Return = ();

    fn execute(&self, _session_id: SessionId) {}
}
