use core::future::ready;

use futures::{pin_mut, select as race, FutureExt, StreamExt};
use nomad::prelude::*;

use crate::config::ConnectorError;
use crate::{Config, SessionId};

/// TODO: docs
pub(crate) struct Session {
    /// TODO: docs
    buffer: Buffer,

    /// TODO: docs
    id: SessionId,

    /// TODO: docs
    receiver: collab::Receiver,

    /// TODO: docs
    _sender: collab::Sender,
}

impl Session {
    /// Returns the [`SessionId`] of the session, which is unique to each
    /// session and can be sent to other peers to join the session.
    pub(crate) fn id(&self) -> SessionId {
        self.id
    }

    /// TODO: docs
    pub async fn join(
        config: Get<Config>,
        session_id: SessionId,
    ) -> Result<Self, JoinError> {
        let (sender, receiver, session) =
            config.get().connector()?.join(session_id.into()).await?;

        Ok(Self {
            buffer: create_buffer(session).await,
            id: session_id,
            receiver,
            _sender: sender,
        })
    }

    /// TODO: docs
    pub async fn run(&mut self) {
        let editor_id = EditorId::generate();

        let edits = self
            .buffer
            .edits()
            .filter(|edit| ready(edit.created_by() != editor_id));

        pin_mut!(edits);

        loop {
            race! {
                maybe_edit = edits.next().fuse() => {
                    let Some(edit) = maybe_edit else { return };
                }
                maybe_msg = self.receiver.recv().fuse() => {
                    let Ok(msg) = maybe_msg else { return };
                },
            }
        }
    }

    /// TODO: docs
    pub async fn start(
        config: Get<Config>,
        buffer: Buffer,
    ) -> Result<Self, StartError> {
        let (sender, receiver, session_id) =
            config.get().connector()?.start().await?;

        Ok(Self { buffer, id: session_id.into(), receiver, _sender: sender })
    }
}

async fn create_buffer(session: collab::messages::Session) -> Buffer {
    todo!()
}

/// Whether there is an active collab session or not.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum SessionState {
    /// There is an active collab session.
    Active(SessionId),

    /// There is no active collab session.
    #[default]
    Inactive,
}

#[derive(Debug, thiserror::Error)]
pub enum JoinError {
    #[error(transparent)]
    Connection(#[from] collab::Error),

    #[error(transparent)]
    Connector(#[from] ConnectorError),
}

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error(transparent)]
    Connection(#[from] collab::Error),

    #[error(transparent)]
    Connector(#[from] ConnectorError),
}
