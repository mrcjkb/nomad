//! TODO: docs.

#![allow(missing_docs)]

use core::convert::Infallible;
use core::ops::{Deref, DerefMut};

use collab_server::SessionId;
use collab_server::message::Message;
use eerie::PeerId;
use nvimx2::backend::{Backend, BackendAdapter};
use nvimx2::{AsyncCtx, fs};

use crate::backend::{
    ActionForSelectedSession,
    CollabBackend,
    CollabBuffer,
    CollabFs,
    StartArgs,
    StartInfos,
    default_read_replica,
    default_search_project_root,
};

#[allow(clippy::type_complexity)]
pub struct CollabTestBackend<T> {
    inner: T,
    confirm_start: Option<Box<dyn FnMut(&fs::AbsPath) -> bool>>,
    clipboard: Option<SessionId>,
}

impl<T> CollabTestBackend<T> {
    pub fn confirm_start_with(
        mut self,
        fun: impl FnMut(&fs::AbsPath) -> bool + 'static,
    ) -> Self
where {
        self.confirm_start = Some(Box::new(fun) as _);
        self
    }

    pub fn new(inner: T) -> Self {
        Self { inner, clipboard: None, confirm_start: None }
    }
}

impl<T: Backend> CollabBackend for CollabTestBackend<T>
where
    T::Fs: CollabFs,
    for<'a> T::Buffer<'a>: CollabBuffer<LspRootError = Infallible>,
{
    type CopySessionIdError = Infallible;
    type ReadReplicaError = Infallible;
    type SearchProjectRootError = Infallible;
    type ServerTx = futures_util::sink::Drain<Message>;
    type ServerRx = futures_util::stream::Pending<Result<Message, Infallible>>;
    type ServerTxError = Infallible;
    type ServerRxError = Infallible;
    type StartSessionError = Infallible;
    type BufferLspRootError = Infallible;

    async fn confirm_start(
        project_root: &fs::AbsPath,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> bool {
        ctx.with_backend(|this| match &mut this.confirm_start {
            Some(fun) => fun(project_root),
            None => true,
        })
    }

    async fn copy_session_id(
        session_id: SessionId,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<(), Self::CopySessionIdError> {
        ctx.with_backend(|this| this.clipboard = Some(session_id));
        Ok(())
    }

    async fn read_replica(
        peer_id: PeerId,
        project_root: &fs::AbsPath,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<eerie::Replica, Self::ReadReplicaError> {
        let _ = default_read_replica::read_replica(
            peer_id,
            project_root.to_owned(),
            ctx,
        )
        .await;
        todo!();
    }

    async fn search_project_root(
        buffer_id: nvimx2::backend::BufferId<Self>,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<eerie::fs::AbsPathBuf, Self::SearchProjectRootError> {
        let _ = default_search_project_root::search(buffer_id, ctx).await;
        todo!()
    }

    async fn select_session<'pairs>(
        _sessions: &'pairs [(fs::AbsPathBuf, SessionId)],
        _action: ActionForSelectedSession,
        _ctx: &mut AsyncCtx<'_, Self>,
    ) -> Option<&'pairs (fs::AbsPathBuf, SessionId)> {
        todo!()
    }

    async fn start_session(
        _args: StartArgs<'_>,
        _ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<StartInfos<Self>, Self::StartSessionError> {
        todo!()
    }
}

impl<T: Backend> BackendAdapter for CollabTestBackend<T> {
    type Base = T;
}

impl<T> Deref for CollabTestBackend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for CollabTestBackend<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl CollabBuffer for &mut nvimx2::tests::buffer::TestBuffer {
    type LspRootError = Infallible;

    fn lsp_root(
        _: Self::Id,
    ) -> Result<Option<fs::AbsPathBuf>, Self::LspRootError> {
        todo!()
    }
}

impl CollabFs for nvimx2::tests::fs::TestFs {
    type HomeDirError = Infallible;

    async fn home_dir(
        &mut self,
    ) -> Result<fs::AbsPathBuf, Self::HomeDirError> {
        todo!()
    }
}

impl<T: Default> Default for CollabTestBackend<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
