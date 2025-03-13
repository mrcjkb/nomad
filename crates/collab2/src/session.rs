use std::io;

use collab_server::client::ClientRxError;
use ed::{AsyncCtx, notify};
use flume::Receiver;
use futures_util::{FutureExt, SinkExt, StreamExt, pin_mut, select};

use crate::backend::{CollabBackend, MessageRx, MessageTx};
use crate::leave::StopSession;
use crate::project::ProjectHandle;

pub(crate) struct Session<B: CollabBackend> {
    args: NewSessionArgs<B>,
}

pub(crate) struct NewSessionArgs<B: CollabBackend> {
    /// TODO: docs..
    pub(crate) message_rx: MessageRx<B>,

    /// TODO: docs..
    pub(crate) message_tx: MessageTx<B>,

    /// TODO: docs.
    pub(crate) project_handle: ProjectHandle<B>,

    /// TODO: docs.
    pub(crate) stop_rx: Receiver<StopSession>,
}

pub(crate) enum RunSessionError {
    Rx(ClientRxError),
    RxExhausted,
    Tx(io::Error),
}

impl<B: CollabBackend> Session<B> {
    pub(crate) fn new(args: NewSessionArgs<B>) -> Self {
        Self { args }
    }

    pub(crate) async fn run(
        self,
        _ctx: &mut AsyncCtx<'_, B>,
    ) -> Result<(), RunSessionError> {
        let NewSessionArgs { stop_rx, message_rx, message_tx, .. } = self.args;

        pin_mut!(message_rx);
        pin_mut!(message_tx);

        loop {
            select! {
                maybe_msg_res = message_rx.next().fuse() => {
                    let msg = maybe_msg_res
                        .ok_or(RunSessionError::RxExhausted)?
                        .map_err(RunSessionError::Rx)?;

                    // Echo it back. Just a placeholder for now.
                    message_tx
                        .send(msg)
                        .await
                        .map_err(RunSessionError::Tx)?;
                },

                _ = stop_rx.recv_async() => {
                    return Ok(());
                },
            }
        }
    }
}

impl notify::Error for RunSessionError {
    fn to_message(&self) -> (notify::Level, notify::Message) {
        match self {
            RunSessionError::Rx(_) => todo!(),
            RunSessionError::RxExhausted => {
                let msg = "the server kicked this peer out of the session";
                (notify::Level::Warn, notify::Message::from_str(msg))
            },
            RunSessionError::Tx(_) => todo!(),
        }
    }
}
