//! TODO: docs.

use editor::Context;
use editor::command::ToCompletionFn;
use editor::module::AsyncAction;

use crate::collab::Collab;
use crate::editors::{ActionForSelectedSession, CollabEditor};
use crate::session::{NoActiveSessionError, SessionInfos, Sessions};

/// TODO: docs.
#[derive(cauchy::Clone)]
pub struct Pause<Ed: CollabEditor> {
    sessions: Sessions<Ed>,
}

impl<Ed: CollabEditor> Pause<Ed> {
    pub(crate) async fn call_inner(
        &self,
        ctx: &mut Context<Ed>,
    ) -> Result<(), PauseError<Ed>> {
        let Some(session_infos) = self
            .sessions
            .select(ActionForSelectedSession::Pause, ctx)
            .await?
            .and_then(|(_, session_id)| self.sessions.get(session_id))
        else {
            return Ok(());
        };

        if session_infos.pause_remote.pause() {
            Ok(())
        } else {
            Err(PauseError::SessionIsAlreadyPaused(session_infos))
        }
    }
}

impl<Ed: CollabEditor> AsyncAction<Ed> for Pause<Ed> {
    const NAME: &str = "pause";

    type Args = ();

    async fn call(&mut self, _: Self::Args, ctx: &mut Context<Ed>) {
        if let Err(err) = self.call_inner(ctx).await {
            Ed::on_pause_error(err, ctx);
        }
    }
}

/// The type of error that can occur when [`Pause`]ing fails.
#[derive(
    cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::PartialEq,
)]
pub enum PauseError<Ed: CollabEditor> {
    /// There are no active sessions to pause.
    #[display("{}", NoActiveSessionError)]
    NoActiveSession,

    /// The session is already paused.
    #[display("The session is already paused")]
    SessionIsAlreadyPaused(SessionInfos<Ed>),
}

impl<Ed: CollabEditor> From<&Collab<Ed>> for Pause<Ed> {
    fn from(collab: &Collab<Ed>) -> Self {
        Self { sessions: collab.sessions.clone() }
    }
}

impl<Ed: CollabEditor> ToCompletionFn<Ed> for Pause<Ed> {
    fn to_completion_fn(&self) {}
}

impl<Ed: CollabEditor> From<NoActiveSessionError> for PauseError<Ed> {
    fn from(_: NoActiveSessionError) -> Self {
        Self::NoActiveSession
    }
}
