//! TODO: docs.

use std::collections::hash_map;
use std::io;

use abs_path::AbsPathBuf;
use collab_server::client as collab_client;
use collab_types::{Message, Peer, PeerId};
use editor::{Access, AccessMut, Context, Shared};
use flume::Receiver;
use futures_util::{FutureExt, SinkExt, StreamExt, pin_mut, select_biased};
use fxhash::FxHashMap;
use smallvec::SmallVec;

use crate::editors::{ActionForSelectedSession, MessageRx, MessageTx};
use crate::event_stream::{EventError, EventStream};
use crate::leave::StopRequest;
use crate::project::{Project, SynchronizeError};
use crate::{CollabEditor, SessionId};

/// TODO: docs.
#[derive(cauchy::Debug, cauchy::Default, cauchy::Clone)]
pub struct Sessions<Ed: CollabEditor> {
    inner: Shared<FxHashMap<SessionId<Ed>, SessionInfos<Ed>>>,
}

/// TODO: docs.
#[derive(cauchy::Debug, cauchy::Clone)]
#[allow(dead_code)]
pub struct SessionInfos<Ed: CollabEditor> {
    /// The [`PeerId`] of the host of the session.
    pub(crate) host_id: PeerId,

    /// TODO: docs..
    pub(crate) local_peer: Peer,

    /// TODO: docs..
    pub(crate) remote_peers: RemotePeers,

    /// The path to the root of the project.
    pub(crate) project_root_path: AbsPathBuf,

    /// The ID of the session.
    pub(crate) session_id: SessionId<Ed>,
}

/// TODO: docs.
#[derive(cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::From)]
#[display("{_0}")]
pub enum SessionError<Ed: CollabEditor> {
    /// TODO: docs.
    EventRx(#[from] EventError<Ed>),

    /// TODO: docs.
    MessageRx(#[from] collab_client::ReceiveError),

    /// TODO: docs.
    #[display("the server kicked this peer out of the session")]
    MessageRxExhausted,

    /// TODO: docs.
    MessageTx(#[from] io::Error),

    /// TODO: docs.
    Synchronize(#[from] SynchronizeError<Ed>),
}

/// TODO: docs.
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq, Eq)]
#[display("there's no active collaborative editing session")]
pub struct NoActiveSessionError;

/// TODO: docs.
pub(crate) struct Session<Ed: CollabEditor> {
    /// TODO: docs.
    pub(crate) event_stream: EventStream<Ed>,

    /// TODO: docs.
    pub(crate) message_rx: MessageRx<Ed>,

    /// TODO: docs.
    pub(crate) message_tx: MessageTx<Ed>,

    /// TODO: docs.
    pub(crate) project: Project<Ed>,

    /// TODO: docs.
    pub(crate) stop_rx: Receiver<StopRequest>,

    /// TODO: docs.
    pub(crate) _remove_on_drop: RemoveOnDrop<Ed>,
}

#[derive(Debug, Clone)]
pub(crate) struct RemotePeers {
    /// A map of all the peers currently in the session.
    ///
    /// It also includes the local peer, so it's guaranteed to never be empty.
    inner: Shared<FxHashMap<PeerId, Peer>>,
}

/// TODO: docs.
pub(crate) struct RemoveOnDrop<Ed: CollabEditor> {
    sessions: Sessions<Ed>,
    session_id: SessionId<Ed>,
}

impl<Ed: CollabEditor> Sessions<Ed> {
    /// Returns the infos for the session with the given ID, if any.
    pub fn get(&self, session_id: SessionId<Ed>) -> Option<SessionInfos<Ed>> {
        self.inner.with(|inner| inner.get(&session_id).cloned())
    }

    /// Inserts the given infos.
    ///
    /// # Panics
    ///
    /// Panics if there are already infos with the same session ID.
    #[track_caller]
    pub(crate) fn insert(&self, infos: SessionInfos<Ed>) -> RemoveOnDrop<Ed> {
        let session_id = infos.session_id;

        self.inner.with_mut(|inner| match inner.entry(session_id) {
            hash_map::Entry::Vacant(vacant) => {
                vacant.insert(infos);
            },
            hash_map::Entry::Occupied(_) => {
                panic!("already have infos for {:?}", infos.session_id)
            },
        });

        RemoveOnDrop { sessions: self.clone(), session_id }
    }

    pub(crate) async fn select(
        &self,
        action: ActionForSelectedSession,
        ctx: &mut Context<Ed>,
    ) -> Result<Option<(AbsPathBuf, SessionId<Ed>)>, NoActiveSessionError>
    {
        let active_sessions = self.inner.with(|map| {
            map.iter()
                .map(|(session_id, infos)| {
                    (infos.project_root_path.clone(), *session_id)
                })
                .collect::<SmallVec<[_; 1]>>()
        });

        let session = match &*active_sessions {
            [] => return Err(NoActiveSessionError),
            [single] => single,
            sessions => {
                match Ed::select_session(sessions, action, ctx).await {
                    Some(session) => session,
                    None => return Ok(None),
                }
            },
        };

        Ok(Some(session.clone()))
    }

    fn remove(&self, session_id: SessionId<Ed>) -> bool {
        self.inner.with_mut(|inner| inner.remove(&session_id).is_some())
    }
}

impl<Ed: CollabEditor> SessionInfos<Ed> {
    /// TODO: docs.
    pub fn id(&self) -> SessionId<Ed> {
        self.session_id
    }
}

impl RemotePeers {
    pub(crate) fn get(&self, peer_id: PeerId) -> Option<Peer> {
        self.inner.with(|inner| inner.get(&peer_id).cloned())
    }

    #[track_caller]
    pub(crate) fn insert(&self, peer: Peer) {
        self.inner.with_mut(|inner| match inner.entry(peer.id) {
            hash_map::Entry::Vacant(vacant) => {
                vacant.insert(peer);
            },
            hash_map::Entry::Occupied(occupied) => {
                panic!(
                    "peer with ID {:?} already exists: {:?}",
                    peer.id,
                    occupied.get()
                )
            },
        });
    }

    #[track_caller]
    pub(crate) fn remove(&self, peer_id: PeerId) -> Peer {
        self.inner.with_mut(|inner| match inner.remove(&peer_id) {
            Some(peer) => peer,
            None => panic!("no peer with ID {:?} exists", peer_id),
        })
    }
}

impl<Ed: CollabEditor> Session<Ed> {
    pub(crate) async fn run(
        self,
        ctx: &mut Context<Ed>,
    ) -> Result<(), SessionError<Ed>> {
        let Self {
            mut event_stream,
            message_rx,
            message_tx,
            mut project,
            stop_rx,
            _remove_on_drop,
        } = self;

        pin_mut!(message_rx);
        pin_mut!(message_tx);

        let mut stop_stream = stop_rx.into_stream();

        loop {
            select_biased! {
                event_res = event_stream.next(ctx).fuse() => {
                    if let Some(message) =
                        project.synchronize(event_res?, ctx).await?
                    {
                        message_tx.send(message).await?;
                    }
                },
                maybe_message_res = message_rx.next() => {
                    let message = maybe_message_res
                        .ok_or(SessionError::MessageRxExhausted)??;

                    if let Message::ProjectRequest(request) = message {
                        let response = project.handle_request(request);
                        let message = Message::ProjectResponse(response);
                        message_tx.send(message).await?;
                        continue;
                    }

                    project.integrate(message, ctx).await;
                },
                stop_request = stop_stream.select_next_some() => {
                    stop_request.send_stopped();
                    return Ok(())
                },
            }
        }
    }
}

impl Access<FxHashMap<PeerId, Peer>> for RemotePeers {
    fn with<R>(&self, fun: impl FnOnce(&FxHashMap<PeerId, Peer>) -> R) -> R {
        self.inner.with(fun)
    }
}

impl AccessMut<FxHashMap<PeerId, Peer>> for RemotePeers {
    fn with_mut<R>(
        &mut self,
        fun: impl FnOnce(&mut FxHashMap<PeerId, Peer>) -> R,
    ) -> R {
        self.inner.with_mut(fun)
    }
}

impl From<collab_types::Peers> for RemotePeers {
    fn from(peers: collab_types::Peers) -> Self {
        Self {
            inner: Shared::new(
                peers.into_iter().map(|peer| (peer.id, peer)).collect(),
            ),
        }
    }
}

impl<Ed: CollabEditor> Drop for RemoveOnDrop<Ed> {
    fn drop(&mut self) {
        assert!(self.sessions.remove(self.session_id));
    }
}
