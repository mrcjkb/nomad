//! TODO: docs.

use core::pin::Pin;
use core::task::{Context, Poll, ready};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::ffi::OsString;
use std::io;
use std::time::SystemTime;

use futures_lite::Stream;
use notify::{RecursiveMode, Watcher};

use crate::fs::{
    AbsPath,
    AbsPathBuf,
    DirEntry,
    Fs,
    FsEvent,
    FsNode,
    FsNodeKind,
    FsNodeName,
    InvalidFsNodeNameError,
    Metadata,
    Symlink,
};

/// TODO: docs.
#[derive(Debug, Default, Copy, Clone)]
pub struct OsFs;

/// TODO: docs.
pub struct OsDirEntry {
    inner: async_fs::DirEntry,
}

/// TODO: docs.
pub struct OsDirectory {
    _metadata: async_fs::Metadata,
}

/// TODO: docs.
pub struct OsFile {
    _metadata: async_fs::Metadata,
}

/// TODO: docs.
pub struct OsSymlink {
    _metadata: async_fs::Metadata,
    path: AbsPathBuf,
}

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct OsReadDir {
        #[pin]
        inner: async_fs::ReadDir,
    }
}

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct OsWatcher {
        buffered: VecDeque<FsEvent<OsFs>>,
        #[pin]
        inner: flume::r#async::RecvStream<
            'static,
            Result<(notify::Event, SystemTime), notify::Error>,
        >,
    }
}

/// TODO: docs.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum OsNameError {
    /// TODO: docs.
    #[error("file name {:?} is not valid UTF-8", .0)]
    NotUtf8(OsString),

    /// TODO: docs.
    #[error(transparent)]
    Invalid(#[from] InvalidFsNodeNameError),
}

impl Fs for OsFs {
    type DirEntry = OsDirEntry;
    type DirEntryError = io::Error;
    type Directory = OsDirectory;
    type File = OsFile;
    type Symlink = OsSymlink;
    type NodeAtPathError = io::Error;
    type ReadDir = OsReadDir;
    type ReadDirError = io::Error;
    type Timestamp = SystemTime;
    type WatchError = notify::Error;
    type Watcher = OsWatcher;

    #[inline]
    async fn node_at_path<P: AsRef<AbsPath>>(
        &self,
        path: P,
    ) -> Result<Option<FsNode<Self>>, Self::NodeAtPathError> {
        let path = path.as_ref();
        let metadata = match async_fs::symlink_metadata(path).await {
            Ok(metadata) => metadata,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        let Ok(file_type) = metadata.file_type().try_into() else {
            return Ok(None);
        };
        Ok(Some(match file_type {
            FsNodeKind::File => FsNode::File(OsFile { _metadata: metadata }),
            FsNodeKind::Directory => {
                FsNode::Directory(OsDirectory { _metadata: metadata })
            },
            FsNodeKind::Symlink => FsNode::Symlink(OsSymlink {
                _metadata: metadata,
                path: path.to_owned(),
            }),
        }))
    }

    #[inline]
    fn now(&self) -> Self::Timestamp {
        SystemTime::now()
    }

    #[inline]
    async fn read_dir<P: AsRef<AbsPath>>(
        &self,
        dir_path: P,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        async_fs::read_dir(dir_path.as_ref())
            .await
            .map(|inner| OsReadDir { inner })
    }

    #[inline]
    async fn watch<P: AsRef<AbsPath>>(
        &self,
        path: P,
    ) -> Result<Self::Watcher, Self::WatchError> {
        let (tx, rx) = flume::unbounded();
        let mut watcher = notify::recommended_watcher(
            move |event_res: Result<_, notify::Error>| {
                let _ =
                    tx.send(event_res.map(|event| (event, SystemTime::now())));
            },
        )?;
        watcher.watch(
            std::path::Path::new(path.as_ref().as_str()),
            RecursiveMode::Recursive,
        )?;
        Ok(OsWatcher {
            buffered: VecDeque::default(),
            inner: rx.into_stream(),
        })
    }
}

impl DirEntry<OsFs> for OsDirEntry {
    type MetadataError = io::Error;
    type NameError = OsNameError;
    type NodeKindError = io::Error;

    #[inline]
    async fn metadata(&self) -> Result<Metadata<OsFs>, Self::MetadataError> {
        self.inner.metadata().await.map(Into::into)
    }

    #[inline]
    async fn name(&self) -> Result<Cow<'_, FsNodeName>, Self::NameError> {
        let os_name = self.inner.file_name();
        let fs_name: &FsNodeName = os_name
            .to_str()
            .ok_or_else(|| OsNameError::NotUtf8(os_name.clone()))?
            .try_into()?;
        Ok(Cow::Owned(fs_name.to_owned()))
    }

    #[inline]
    async fn node_kind(
        &self,
    ) -> Result<Option<FsNodeKind>, Self::NodeKindError> {
        self.inner.file_type().await.map(|file_type| file_type.try_into().ok())
    }
}

impl Stream for OsReadDir {
    type Item = Result<OsDirEntry, io::Error>;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        match ready!(self.project().inner.poll_next(ctx)) {
            Some(Ok(entry)) => {
                Poll::Ready(Some(Ok(OsDirEntry { inner: entry })))
            },
            Some(Err(err)) => Poll::Ready(Some(Err(err))),
            None => Poll::Ready(None),
        }
    }
}

impl Symlink<OsFs> for OsSymlink {
    type FollowError = io::Error;

    #[inline]
    async fn follow(&self) -> Result<Option<FsNode<OsFs>>, Self::FollowError> {
        let target_path = async_fs::read_link(&*self.path).await?;
        let path = <&AbsPath>::try_from(&*target_path)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        OsFs.node_at_path(path).await
    }

    #[inline]
    async fn follow_recursively(
        &self,
    ) -> Result<Option<FsNode<OsFs>>, Self::FollowError> {
        let target_path = async_fs::canonicalize(&*self.path).await?;
        let path = <&AbsPath>::try_from(&*target_path)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        OsFs.node_at_path(path).await
    }
}

impl Stream for OsWatcher {
    type Item = Result<FsEvent<OsFs>, notify::Error>;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            if let Some(event) = this.buffered.pop_front() {
                return Poll::Ready(Some(Ok(event)));
            }
            let Some((event, timestamp)) =
                ready!(this.inner.as_mut().poll_next(ctx)).transpose()?
            else {
                return Poll::Ready(None);
            };
            this.buffered.extend(FsEvent::from_notify(event, timestamp));
        }
    }
}
