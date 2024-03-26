use nomad::prelude::*;

use crate::{Collab, Config, Context, SessionId};

#[derive(Clone)]
pub(crate) struct Join {
    _config: Get<Config>,
}

impl Join {
    pub(crate) fn new(ctx: &Context) -> Self {
        Self { _config: ctx.config.clone() }
    }
}

#[async_action]
impl Action<Collab> for Join {
    const NAME: ActionName = action_name!("join");

    type Args = SessionId;

    type Return = ();

    async fn execute(&self, _session_id: SessionId) {}
}
