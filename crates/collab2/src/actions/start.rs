use collab_server::message::Message;
use futures_util::StreamExt;
use nomad::ctx::NeovimCtx;
use nomad::{action_name, ActionName, AsyncAction, Shared};

use super::UserBusyError;
use crate::session::Session;
use crate::session_status::SessionStatus;
use crate::Collab;

#[derive(Clone)]
pub(crate) struct Start {
    session_status: Shared<SessionStatus>,
}

impl Start {
    pub(crate) fn new(session_status: Shared<SessionStatus>) -> Self {
        Self { session_status }
    }
}

impl AsyncAction for Start {
    const NAME: ActionName = action_name!("start");
    type Args = ();
    type Docs = ();
    type Module = Collab;

    async fn execute(
        &mut self,
        _: Self::Args,
        ctx: NeovimCtx<'_>,
    ) -> Result<(), UserBusyError<true>> {
        #[rustfmt::skip]
        Starter::new(self.session_status.clone(), ctx.to_static())?
            .find_project_root().await?
            .confirm_start().await?
            .read_project().await?
            .connect_to_server().await?
            .authenticate(()).await?
            .start_session().await?
            .run_session().await?;

        Ok(())
    }

    fn docs(&self) -> Self::Docs {}
}

struct Starter<State> {
    state: State,
}

impl<State> From<State> for Starter<State> {
    fn from(state: State) -> Self {
        Self { state }
    }
}

// match self.session_status.with(|s| UserBusyError::try_from(s)).ok() {
//     Some(err) => return Err(err),
//     _ => self.session_status.set(SessionStatus::Starting),
// }
//
// let mut session = Session::start().await;
// self.session_status.set(SessionStatus::InSession(session.project()));
// ctx.spawn(async move {
//     let (tx, rx) = flume::unbounded::<Message>();
//     let tx = tx.into_sink::<'static>();
//     let rx = rx
//         .into_stream::<'static>()
//         .map(Ok::<_, core::convert::Infallible>);
//     let _err = session.run(tx, rx).await;
// });
