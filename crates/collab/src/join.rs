use nomad::prelude::*;

use crate::{Collab, Config, Context, Session, SessionId, SessionState};

#[derive(Clone)]
pub(crate) struct Join {
    config: Get<Config>,
    state: Get<SessionState>,
    set_state: Set<SessionState>,
}

impl Join {
    pub(crate) fn new(ctx: &Context) -> Self {
        Self {
            config: ctx.config.clone(),
            state: ctx.state.clone(),
            set_state: ctx.set_state.clone(),
        }
    }
}

#[async_action]
impl Action<Collab> for Join {
    const NAME: ActionName = action_name!("join");

    type Args = SessionId;

    type Return = ();

    async fn execute(&self, session_id: SessionId) -> Result<(), JoinError> {
        if let &SessionState::Active(session_id) = self.state.get() {
            return Err(JoinError::ExistingSession(session_id));
        }

        let mut session =
            Session::join(self.config.clone(), session_id).await?;

        self.set_state.set(SessionState::Active(session.id()));

        let _ = session.run().await;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JoinError {
    #[error("there is already an active session with ID {0}")]
    ExistingSession(SessionId),

    #[error(transparent)]
    Join(#[from] crate::session::JoinError),
}

impl From<JoinError> for WarningMsg {
    fn from(err: JoinError) -> Self {
        let mut msg = WarningMsg::new();
        msg.add(err.to_string());
        msg
    }
}
