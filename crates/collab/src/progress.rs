//! Contains types and traits related to reporting the progress of long-running
//! operations to the user.

use std::borrow::Cow;

use abs_path::{AbsPath, NodeName};
use editor::Context;

use crate::{CollabEditor, config, join, start};

/// A trait for types that can report the progress of long-running operations
/// to the user.
///
/// Editors that don't support progress reporting can set their
/// [`ProgressReporter`](CollabEditor::ProgressReporter) to `()`, which
/// implements this trait for all [`CollabEditor`]s by simply doing nothing.
pub trait ProgressReporter<Ed: CollabEditor> {
    /// Returns a new instance of the reporter.
    fn new(ctx: &mut Context<Ed>) -> Self;

    /// Reports a progress update for the [`Join`](crate::join::Join) action.
    fn report_join_progress(
        &mut self,
        state: JoinState<'_, Ed>,
        ctx: &mut Context<Ed>,
    );

    /// Reports a progress update for the [`Start`](crate::start::Start)
    /// action.
    fn report_start_progress(
        &mut self,
        state: StartState<'_, Ed>,
        ctx: &mut Context<Ed>,
    );
}

/// An enum representing the different progress states of the
/// [`Join`](join::Join) action.
///
/// The variants form a linear sequence, and each variant is guaranteed to be
/// followed by either:
///
/// * another instance of the same variant;
/// * the next variant in the sequence;
/// * a `Done(Err(..))` if an error occurred;
pub enum JoinState<'a, Ed: CollabEditor> {
    /// The client is connecting to the server at the given address.
    ConnectingToServer(config::ServerAddress<'a>),

    /// The client has connected to the server, and is now waiting for it to
    /// respond with a [`Welcome`](collab_server::client::Welcome) message.
    JoiningSession,

    /// We've received the [`Welcome`](collab_server::client::Welcome) message,
    /// and are now waiting to receive the project with the given name from
    /// another peer in the session.
    ReceivingProject(Cow<'a, NodeName>),

    /// We've received the project, and are now writing it to disk under the
    /// directory at the given path.
    WritingProject(Cow<'a, AbsPath>),

    /// Either the project has been written, or an error occurred.
    Done(Result<(), join::JoinError<Ed>>),
}

/// An enum representing the different progress states of the
/// [`Start`](start::Start) action.
///
/// The variants form a linear sequence, and each variant is guaranteed to be
/// followed by either:
///
/// * another instance of the same variant;
/// * the next variant in the sequence;
/// * a `Done(Err(..))` if an error occurred;
pub enum StartState<'a, Ed: CollabEditor> {
    /// The client is connecting to the server at the given address.
    ConnectingToServer(config::ServerAddress<'a>),

    /// The client has connected to the server, and is now waiting for it to
    /// respond with a [`Welcome`](collab_server::client::Welcome) message.
    StartingSession,

    /// We've received the [`Welcome`](collab_server::client::Welcome) message,
    /// and are now reading the project rooted at the given path.
    ReadingProject(Cow<'a, AbsPath>),

    /// Either the project has been read, or an error occurred.
    Done(Result<(), start::StartError<Ed>>),
}

impl<Ed: CollabEditor> JoinState<'_, Ed> {
    /// Returns a `'static` version of this [`JoinState`].
    pub fn to_owned(&self) -> JoinState<'static, Ed>
    where
        join::JoinError<Ed>: Clone,
    {
        match self {
            Self::ConnectingToServer(server_addr) => {
                JoinState::ConnectingToServer(server_addr.to_owned())
            },
            Self::JoiningSession => JoinState::JoiningSession,
            Self::ReceivingProject(project_name) => {
                JoinState::ReceivingProject(Cow::Owned(
                    project_name.clone().into_owned(),
                ))
            },
            Self::WritingProject(root_path) => JoinState::WritingProject(
                Cow::Owned(root_path.clone().into_owned()),
            ),
            Self::Done(res) => JoinState::Done(res.clone()),
        }
    }
}

impl<Ed: CollabEditor> StartState<'_, Ed> {
    /// Returns a `'static` version of this [`StartState`].
    pub fn to_owned(&self) -> StartState<'static, Ed>
    where
        start::StartError<Ed>: Clone,
    {
        match self {
            Self::ConnectingToServer(server_addr) => {
                StartState::ConnectingToServer(server_addr.to_owned())
            },
            Self::StartingSession => StartState::StartingSession,
            Self::ReadingProject(root_path) => StartState::ReadingProject(
                Cow::Owned(root_path.clone().into_owned()),
            ),
            Self::Done(res) => StartState::Done(res.clone()),
        }
    }
}

impl<Ed: CollabEditor> ProgressReporter<Ed> for () {
    fn new(_: &mut Context<Ed>) -> Self {}

    fn report_join_progress(
        &mut self,
        _: JoinState<'_, Ed>,
        _: &mut Context<Ed>,
    ) {
    }

    fn report_start_progress(
        &mut self,
        _: StartState<Ed>,
        _: &mut Context<Ed>,
    ) {
    }
}
