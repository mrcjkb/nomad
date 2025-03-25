use core::error::Error;
use core::fmt;
use core::pin::Pin;

use ed::fs::{
    self,
    AbsPath,
    AbsPathBuf,
    Directory,
    Metadata,
    MetadataNameError,
};
use futures_util::stream::{self, FusedStream, StreamExt};
use futures_util::{FutureExt, pin_mut, select};

use crate::filter::{Filter, Filtered};

/// TODO: docs.
pub trait WalkDir<Fs: fs::Fs>: Sized {
    /// The type of error that can occur when reading a directory fails.
    type ReadError: Error;

    /// The type of error that can occur when reading a specific entry in a
    /// directory fails.
    type ReadEntryError: Error;

    /// TODO: docs.
    fn read_dir(
        &self,
        dir_path: &AbsPath,
    ) -> impl Future<
        Output = Result<
            impl FusedStream<Item = Result<Fs::Metadata, Self::ReadEntryError>>,
            Self::ReadError,
        >,
    >;

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter<Fs>,
    {
        Filtered::new(filter, self)
    }

    /// TODO: docs.
    #[allow(clippy::type_complexity)]
    #[inline]
    fn for_each<'a, H, E>(
        &'a self,
        dir_path: &'a AbsPath,
        handler: H,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalkError<Fs, Self, E>>> + 'a>>
    where
        H: AsyncFn(&AbsPath, Fs::Metadata) -> Result<(), E> + Clone + 'a,
        E: 'a,
    {
        Box::pin(async move {
            let entries =
                self.read_dir(dir_path).await.map_err(WalkError::ReadDir)?;
            let mut handle_entries = stream::FuturesUnordered::new();
            let mut read_children = stream::FuturesUnordered::new();
            pin_mut!(entries);
            loop {
                select! {
                    res = entries.select_next_some() => {
                        let entry = res.map_err(WalkError::ReadEntry)?;
                        let node_kind = entry.node_kind();
                        if node_kind.is_dir() {
                            let dir_name = entry
                                .name()
                                .map_err(WalkError::NodeName)?;
                            let dir_path = dir_path.join(&dir_name);
                            let handler = handler.clone();
                            read_children.push(async move {
                                self.for_each(&dir_path, handler).await
                            });
                        }
                        let handler = handler.clone();
                        handle_entries.push(async move {
                            handler(dir_path, entry).await
                        });
                    },
                    res = read_children.select_next_some() => res?,
                    res = handle_entries.select_next_some() => {
                        res.map_err(WalkError::Other)?;
                    },
                    complete => return Ok(()),
                }
            }
        })
    }

    /// TODO: docs.
    #[inline]
    fn paths<'a>(
        &'a self,
        dir_path: &'a AbsPath,
    ) -> impl FusedStream<
        Item = Result<AbsPathBuf, WalkError<Fs, Self, MetadataNameError>>,
    > + 'a {
        self.to_stream(dir_path, async |dir_path, entry| {
            entry.name().map(|name| dir_path.join(name))
        })
    }

    /// TODO: docs.
    #[inline]
    fn to_stream<'a, H, T, E>(
        &'a self,
        dir_path: &'a AbsPath,
        handler: H,
    ) -> impl FusedStream<Item = Result<T, WalkError<Fs, Self, E>>> + 'a
    where
        H: AsyncFn(&AbsPath, Fs::Metadata) -> Result<T, E> + Clone + 'a,
        T: 'a,
        E: 'a,
    {
        let (tx, rx) = flume::unbounded();
        let for_each = self
            .for_each(dir_path, async move |dir_path, entry| {
                let _ = tx.send(handler(dir_path, entry).await?);
                Ok(())
            })
            .boxed_local()
            .fuse();
        futures_util::stream::unfold(
            (for_each, rx),
            move |(mut for_each, rx)| async move {
                let res = select! {
                    res = for_each => match res {
                        Ok(()) => return None,
                        Err(err) => Err(err),
                    },
                    res = rx.recv_async() => match res {
                        Ok(value) => Ok(value),
                        Err(_err) => return None,
                    },
                };
                Some((res, (for_each, rx)))
            },
        )
    }
}

/// TODO: docs.
pub enum WalkError<Fs, W, T>
where
    Fs: fs::Fs,
    W: WalkDir<Fs>,
{
    /// TODO: docs.
    Other(T),

    /// TODO: docs.
    NodeName(MetadataNameError),

    /// TODO: docs.
    ReadDir(W::ReadError),

    /// TODO: docs.
    ReadEntry(W::ReadEntryError),
}

/// TODO: docs.
#[derive(derive_more::Debug)]
#[debug(bound(Fs: fs::Fs))]
pub enum FsReadDirError<Fs: fs::Fs> {
    /// TODO: docs.
    NoNodeAtPath,

    /// TODO: docs.
    NodeAtPath(Fs::NodeAtPathError),

    /// TODO: docs.
    ReadDir(<Fs::Directory as fs::Directory>::ReadError),

    /// TODO: docs.
    ReadFile,

    /// TODO: docs.
    ReadSymlink,
}

impl<Fs: fs::Fs> WalkDir<Self> for Fs {
    type ReadError = FsReadDirError<Self>;
    type ReadEntryError = <Fs::Directory as fs::Directory>::ReadEntryError;

    async fn read_dir(
        &self,
        dir_path: &fs::AbsPath,
    ) -> Result<
        impl FusedStream<
            Item = Result<<Self as fs::Fs>::Metadata, Self::ReadEntryError>,
        >,
        Self::ReadError,
    > {
        let Some(node) = self
            .node_at_path(dir_path)
            .await
            .map_err(FsReadDirError::NodeAtPath)?
        else {
            return Err(FsReadDirError::NoNodeAtPath);
        };

        match node {
            fs::FsNode::Directory(dir) => dir
                .read()
                .await
                .map(StreamExt::fuse)
                .map_err(FsReadDirError::ReadDir),
            fs::FsNode::File(_) => Err(FsReadDirError::ReadFile),
            fs::FsNode::Symlink(_) => Err(FsReadDirError::ReadSymlink),
        }
    }
}

impl<Fs, W, T> PartialEq for WalkError<Fs, W, T>
where
    Fs: fs::Fs,
    W: WalkDir<Fs>,
    T: PartialEq,
    W::ReadError: PartialEq,
    W::ReadEntryError: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        use WalkError::*;

        match (self, other) {
            (Other(l), Other(r)) => l == r,
            (NodeName(l), NodeName(r)) => l == r,
            (ReadDir(l), ReadDir(r)) => l == r,
            (ReadEntry(l), ReadEntry(r)) => l == r,
            _ => false,
        }
    }
}

impl<Fs, W, T> fmt::Debug for WalkError<Fs, W, T>
where
    Fs: fs::Fs,
    W: WalkDir<Fs>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(err) => fmt::Debug::fmt(err, f),
            Self::NodeName(err) => fmt::Debug::fmt(err, f),
            Self::ReadDir(err) => fmt::Debug::fmt(err, f),
            Self::ReadEntry(err) => fmt::Debug::fmt(err, f),
        }
    }
}

impl<Fs, W, T> fmt::Display for WalkError<Fs, W, T>
where
    Fs: fs::Fs,
    W: WalkDir<Fs>,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(err) => fmt::Display::fmt(err, f),
            Self::NodeName(err) => fmt::Display::fmt(err, f),
            Self::ReadDir(err) => fmt::Display::fmt(err, f),
            Self::ReadEntry(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl<Fs, W, T> Error for WalkError<Fs, W, T>
where
    Fs: fs::Fs,
    W: WalkDir<Fs>,
    T: Error,
{
}

impl<Fs: fs::Fs> PartialEq for FsReadDirError<Fs>
where
    Fs::NodeAtPathError: PartialEq,
    <Fs::Directory as fs::Directory>::ReadError: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        use FsReadDirError::*;

        match (self, other) {
            (NoNodeAtPath, NoNodeAtPath) => true,
            (NodeAtPath(l), NodeAtPath(r)) => l == r,
            (ReadDir(l), ReadDir(r)) => l == r,
            (ReadFile, ReadFile) => true,
            (ReadSymlink, ReadSymlink) => true,
            _ => false,
        }
    }
}

impl<Fs: fs::Fs> fmt::Display for FsReadDirError<Fs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsReadDirError::NoNodeAtPath => {
                write!(f, "no node at path")
            },
            FsReadDirError::NodeAtPath(err) => fmt::Display::fmt(err, f),
            FsReadDirError::ReadDir(err) => fmt::Display::fmt(err, f),
            FsReadDirError::ReadFile => {
                write!(f, "couldn't read file at path")
            },
            FsReadDirError::ReadSymlink => {
                write!(f, "couldn't read symlink at path")
            },
        }
    }
}

impl<Fs: fs::Fs> Error for FsReadDirError<Fs> {}
