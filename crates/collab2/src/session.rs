use collab_server::SessionId;
use collab_server::message::{Peer, Peers};
use eerie::Replica;
use nvimx2::fs::AbsPathBuf;
use nvimx2::{AsyncCtx, notify};

use crate::CollabBackend;

pub(crate) struct Session<B: CollabBackend> {
    _args: NewSessionArgs<B>,
}

pub(crate) struct NewSessionArgs<B: CollabBackend> {
    /// Whether the [`local_peer`](Self::local_peer) is the host of the
    /// session.
    pub(crate) is_host: bool,

    /// The local [`Peer`].
    pub(crate) local_peer: Peer,

    /// The remote [`Peers`].
    pub(crate) remote_peers: Peers,

    /// The absolute path to the directory containing the project.
    ///
    /// The contents of the directory are assumed to be in sync with with the
    /// [`replica`](Self::replica).
    pub(crate) project_root: AbsPathBuf,

    /// The [`replica`](Self::replica) of the project.
    ///
    /// The files and directories in it are assumed to be in sync with the
    /// contents of the [`project_root`](Self::project_root).
    pub(crate) replica: Replica,

    /// The ID of the session.
    pub(crate) session_id: SessionId,

    /// TODO: docs..
    pub(crate) server_tx: B::ServerTx,

    /// TODO: docs..
    pub(crate) server_rx: B::ServerRx,
}

pub(crate) enum RunSessionError<B: CollabBackend> {
    Tx(B::ServerTxError),
    Rx(B::ServerRxError),
}

impl<B: CollabBackend> Session<B> {
    pub(crate) fn new(args: NewSessionArgs<B>) -> Self {
        Self { _args: args }
    }

    pub(crate) async fn run(
        self,
        _ctx: &mut AsyncCtx<'_, B>,
    ) -> Result<(), RunSessionError<B>> {
        todo!();
    }
}

impl<B: CollabBackend> notify::Error for RunSessionError<B> {
    fn to_message(&self) -> (notify::Level, notify::Message) {
        match self {
            RunSessionError::Tx(err) => err.to_message(),
            RunSessionError::Rx(err) => err.to_message(),
        }
    }
}
