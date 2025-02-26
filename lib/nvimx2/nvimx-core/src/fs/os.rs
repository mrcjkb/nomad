//! TODO: docs.

use core::cell::RefCell;
use core::convert::Infallible;
use core::pin::Pin;
use core::task::{Context, Poll, ready};
use std::collections::VecDeque;
use std::ffi::OsString;
use std::io;
use std::time::SystemTime;

use futures_lite::{Stream, StreamExt};
use notify::{RecursiveMode, Watcher};

use crate::ByteOffset;
use crate::fs::{
    AbsPath,
    AbsPathBuf,
    Directory,
    File,
    Fs,
    FsEvent,
    FsNode,
    FsNodeKind,
    FsNodeNameBuf,
    InvalidFsNodeNameError,
    Metadata,
    Symlink,
};

/// TODO: docs.
#[derive(Debug, Default, Copy, Clone)]
pub struct OsFs {}

/// TODO: docs.
pub struct OsDirectory {
    metadata: LazyOsMetadata,
}

/// TODO: docs.
pub struct OsFile {
    metadata: LazyOsMetadata,
}

/// TODO: docs.
pub struct OsSymlink {
    _metadata: async_fs::Metadata,
    path: AbsPathBuf,
}

/// TODO: docs.
pub struct OsMetadata {
    metadata: async_fs::Metadata,
    node_name: OsString,
}

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct OsWatcher {
        buffered: VecDeque<FsEvent<SystemTime>>,
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

struct LazyOsMetadata {
    metadata: RefCell<Option<async_fs::Metadata>>,
    path: AbsPathBuf,
}

impl LazyOsMetadata {
    fn lazy(path: AbsPathBuf) -> Self {
        Self { metadata: RefCell::new(None), path }
    }

    fn new(metadata: async_fs::Metadata, path: AbsPathBuf) -> Self {
        Self { metadata: RefCell::new(Some(metadata)), path }
    }

    async fn with<R>(
        &self,
        fun: impl FnOnce(&async_fs::Metadata) -> R,
    ) -> Result<R, io::Error> {
        if let Some(meta) = &*self.metadata.borrow() {
            return Ok(fun(meta));
        }
        let metadata = async_fs::metadata(&*self.path).await?;
        *self.metadata.borrow_mut() = Some(metadata);
        Ok(fun(self.metadata.borrow().as_ref().expect("just set it")))
    }
}

impl Fs for OsFs {
    type Directory = OsDirectory;
    type File = OsFile;
    type Symlink = OsSymlink;
    type Timestamp = SystemTime;
    type Watcher = OsWatcher;

    type NodeAtPathError = io::Error;
    type WatchError = notify::Error;

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
            FsNodeKind::File => FsNode::File(OsFile {
                metadata: LazyOsMetadata::new(metadata, path.to_owned()),
            }),
            FsNodeKind::Directory => FsNode::Directory(OsDirectory {
                metadata: LazyOsMetadata::new(metadata, path.to_owned()),
            }),
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

impl Directory for OsDirectory {
    type Fs = OsFs;
    type Metadata = OsMetadata;
    type ReadEntryError = io::Error;
    type ReadError = io::Error;

    async fn read(
        &self,
    ) -> Result<
        impl Stream<Item = Result<OsMetadata, Self::ReadEntryError>> + use<>,
        Self::ReadError,
    > {
        async_fs::read_dir(self.path()).await.map(|read_dir| {
            read_dir.map(|res| {
                res.map(|dir_entry| OsMetadata {
                    metadata: todo!(),
                    node_name: dir_entry.file_name(),
                })
            })
        })
    }

    async fn parent(&self) -> Option<Self> {
        self.path().parent().map(|parent| Self {
            metadata: LazyOsMetadata::lazy(parent.to_owned()),
        })
    }

    fn path(&self) -> &AbsPath {
        &self.metadata.path
    }
}

impl File for OsFile {
    type Fs = OsFs;
    type Error = io::Error;

    async fn len(&self) -> Result<ByteOffset, Self::Error> {
        self.metadata.with(|meta| meta.len().into()).await
    }

    async fn parent(&self) -> <Self::Fs as Fs>::Directory {
        OsDirectory {
            metadata: LazyOsMetadata::lazy(
                self.path().parent().expect("has a parent").to_owned(),
            ),
        }
    }

    fn path(&self) -> &AbsPath {
        &self.metadata.path
    }
}

impl Symlink for OsSymlink {
    type Fs = OsFs;
    type FollowError = io::Error;

    #[inline]
    async fn follow(&self) -> Result<Option<FsNode<OsFs>>, Self::FollowError> {
        let target_path = async_fs::read_link(&*self.path).await?;
        let path = <&AbsPath>::try_from(&*target_path)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        OsFs::default().node_at_path(path).await
    }

    #[inline]
    async fn follow_recursively(
        &self,
    ) -> Result<Option<FsNode<OsFs>>, Self::FollowError> {
        let target_path = async_fs::canonicalize(&*self.path).await?;
        let path = <&AbsPath>::try_from(&*target_path)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        OsFs::default().node_at_path(path).await
    }
}

impl Stream for OsWatcher {
    type Item = Result<FsEvent<SystemTime>, notify::Error>;

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

impl Metadata for OsMetadata {
    type Timestamp = SystemTime;
    type Error = io::Error;
    type NameError = OsNameError;
    type NodeKindError = Infallible;

    async fn created_at(&self) -> Result<Option<SystemTime>, Self::Error> {
        Ok(self.metadata.created().ok())
    }

    async fn last_modified_at(
        &self,
    ) -> Result<Option<SystemTime>, Self::Error> {
        Ok(self.metadata.modified().ok())
    }

    async fn name(&self) -> Result<FsNodeNameBuf, Self::NameError> {
        self.node_name
            .to_str()
            .ok_or_else(|| OsNameError::NotUtf8(self.node_name.clone()))?
            .parse()
            .map_err(OsNameError::Invalid)
    }

    async fn node_kind(&self) -> Result<FsNodeKind, Self::NodeKindError> {
        let file_type = self.metadata.file_type();

        Ok(if file_type.is_dir() {
            FsNodeKind::Directory
        } else if file_type.is_file() {
            FsNodeKind::File
        } else if file_type.is_symlink() {
            FsNodeKind::Symlink
        } else {
            unreachable!("checked when creating it")
        })
    }
}
