//! TODO: docs.

use editor::Context;
use editor::command::ToCompletionFn;
use editor::module::AsyncAction;
use flume::Sender;

use crate::collab::Collab;
use crate::editors::{ActionForSelectedSession, CollabEditor};
use crate::session::{NoActiveSessionError, Sessions};

/// TODO: docs.
#[derive(cauchy::Clone)]
pub struct Leave<Ed: CollabEditor> {
    sessions: Sessions<Ed>,
}

pub(crate) struct StopRequest {
    stopped_tx: Sender<()>,
}

impl<Ed: CollabEditor> Leave<Ed> {
    pub(crate) async fn call_inner(
        &self,
        ctx: &mut Context<Ed>,
    ) -> Result<(), LeaveError> {
        let Some(sesh) = self
            .sessions
            .select(ActionForSelectedSession::Leave, ctx)
            .await?
            .and_then(|(_, session_id)| self.sessions.get(session_id))
        else {
            return Ok(());
        };

        let (stopped_tx, stopped_rx) = flume::bounded(1);

        // Wait for the session to receive the stop request and actually stop.
        if sesh.stop_tx.send_async(StopRequest { stopped_tx }).await.is_ok() {
            let _ = stopped_rx.recv_async().await;
        }

        Ok(())
    }
}

impl<Ed: CollabEditor> AsyncAction<Ed> for Leave<Ed> {
    const NAME: &str = "leave";

    type Args = ();

    async fn call(&mut self, _: Self::Args, ctx: &mut Context<Ed>) {
        if let Err(err) = self.call_inner(ctx).await {
            Ed::on_leave_error(err, ctx);
        }
    }
}

/// The type of error that can occur when [`Leave`]ing fails.
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq)]
pub enum LeaveError {
    /// TODO: docs.
    #[display("{}", NoActiveSessionError)]
    NoActiveSession,
}

impl StopRequest {
    pub(crate) fn send_stopped(self) {
        self.stopped_tx.send(()).expect("rx is still alive");
    }
}

impl<Ed: CollabEditor> From<&Collab<Ed>> for Leave<Ed> {
    fn from(collab: &Collab<Ed>) -> Self {
        Self { sessions: collab.sessions.clone() }
    }
}

impl<Ed: CollabEditor> ToCompletionFn<Ed> for Leave<Ed> {
    fn to_completion_fn(&self) {}
}

impl From<NoActiveSessionError> for LeaveError {
    fn from(_: NoActiveSessionError) -> Self {
        Self::NoActiveSession
    }
}
