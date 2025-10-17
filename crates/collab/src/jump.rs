//! TODO: docs.

use collab_types::{GitHubHandle, PeerHandle};
use editor::command::{self, CompletionFn, ToCompletionFn};
use editor::module::AsyncAction;
use editor::{ByteOffset, Context};
use smallvec::SmallVec;

use crate::collab::Collab;
use crate::editors::{ActionForSelectedSession, CollabEditor};
use crate::session::{NoActiveSessionError, SessionInfos, Sessions};

/// TODO: docs.
#[derive(cauchy::Clone)]
pub struct Jump<Ed: CollabEditor> {
    sessions: Sessions<Ed>,
}

impl<Ed: CollabEditor> Jump<Ed> {
    pub(crate) async fn call_inner(
        &self,
        _peer_handle: PeerHandle,
        _ctx: &mut Context<Ed>,
    ) -> Result<(), JumpError<Ed>> {
        todo!();
    }
}

impl<Ed: CollabEditor> AsyncAction<Ed> for Jump<Ed> {
    const NAME: &str = "jump";

    type Args = command::Parse<GitHubHandle>;

    async fn call(
        &mut self,
        command::Parse(github_handle): Self::Args,
        ctx: &mut Context<Ed>,
    ) {
        if let Err(err) =
            self.call_inner(PeerHandle::GitHub(github_handle), ctx).await
        {
            Ed::on_jump_error(err, ctx);
        }
    }
}

/// The type of error that can occur when [`Jump`]ing fails.
#[derive(
    cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::PartialEq,
)]
pub enum JumpError<Ed: CollabEditor> {
    /// There are no active sessions.
    #[display("{}", NoActiveSessionError)]
    NoActiveSession,

    /// The given peer doesn't have a cursor in the project tracked by the
    /// given session.
    #[display("{_0}'s cursor is not in {}", _1.proj_name())]
    PeerCursorNotInProject(PeerHandle, SessionInfos<Ed>),

    /// There's no peer with the given handle in any of the sessions.
    #[display("There's no peer with handle '{_0}' in any of the sessions")]
    UnknownPeer(PeerHandle),
}

impl<Ed: CollabEditor> From<&Collab<Ed>> for Jump<Ed> {
    fn from(collab: &Collab<Ed>) -> Self {
        Self { sessions: collab.sessions.clone() }
    }
}

impl<Ed: CollabEditor> ToCompletionFn<Ed> for Jump<Ed> {
    fn to_completion_fn(&self) -> impl CompletionFn + 'static {
        |_command_args: command::CommandArgs<'_>, _byte_offset: ByteOffset| {
            SmallVec::<[command::CommandCompletion; 2]>::new()
        }
    }
}

impl<Ed: CollabEditor> From<NoActiveSessionError> for JumpError<Ed> {
    fn from(_: NoActiveSessionError) -> Self {
        Self::NoActiveSession
    }
}
