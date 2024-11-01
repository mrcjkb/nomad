use collab_server::message::Message;
use futures_util::StreamExt;
use nomad::ctx::NeovimCtx;
use nomad::{action_name, ActionName, AsyncAction, Shared};

use super::UserBusyError;
use crate::session::Session;
use crate::session_id::SessionId;
use crate::session_status::SessionStatus;
use crate::Collab;

#[derive(Clone)]
pub(crate) struct Join {
    session_status: Shared<SessionStatus>,
}

impl Join {
    pub(crate) fn new(session_status: Shared<SessionStatus>) -> Self {
        Self { session_status }
    }
}

impl AsyncAction for Join {
    const NAME: ActionName = action_name!("join");
    type Args = SessionId;
    type Docs = ();
    type Module = Collab;

    async fn execute(
        &mut self,
        session_id: Self::Args,
        ctx: NeovimCtx<'_>,
    ) -> Result<(), UserBusyError<false>> {
        #[rustfmt::skip]
        Joiner::new(self.session_status.clone(), ctx.to_static())?
            .connect_to_server().await?
            .authenticate(()).await?
            .join_session(session_id).await?
            .confirm_join().await?
            .request_project().await?
            .find_project_root().await?
            .flush_project().await?
            .jump_to_host().await?
            .run_session().await?;

        Ok(())
    }

    fn docs(&self) {}
}

struct Joiner<State> {
    state: State,
}

impl<State> From<State> for Joiner<State> {
    fn from(state: State) -> Self {
        Self { state }
    }
}

// match self.session_status.with(|s| UserBusyError::try_from(s)).ok() {
//     Some(err) => return Err(err),
//     _ => self.session_status.set(SessionStatus::Joining(session_id)),
// }
//
// let mut session = Session::join().await;
// self.session_status.set(SessionStatus::InSession(session.project()));
// ctx.spawn(async move {
//     let (tx, rx) = flume::unbounded::<Message>();
//     let tx = tx.into_sink::<'static>();
//     let rx = rx
//         .into_stream::<'static>()
//         .map(Ok::<_, core::convert::Infallible>);
//     let _err = session.run(tx, rx).await;
// });
