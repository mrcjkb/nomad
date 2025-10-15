use core::pin::Pin;
use core::task::{Context, Poll};
use std::path::PathBuf;
use std::{ffi, io};

use abs_path::{AbsPath, AbsPathBuf, NodeName};
use futures_util::stream::{self, FusedStream, Stream};

use crate::file_descriptor_permit::FileDescriptorPermit;
use crate::{File, IoErrorExt, Metadata, RealFs, Symlink};

/// TODO: docs.
#[derive(Clone)]
pub struct Directory {
    pub(crate) metadata: async_fs::Metadata,
    pub(crate) path: AbsPathBuf,
}

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct ListMetas {
        dir_path: AbsPathBuf,
        #[pin]
        get_metadata: stream::FuturesUnordered<GetMetadata>,
        #[pin]
        read_dir: async_fs::ReadDir,
        read_dir_is_terminated: bool,
        _fd_permit: FileDescriptorPermit,
    }
}

pin_project_lite::pin_project! {
    struct GetMetadata {
        #[pin]
        symlink_metadata: Pin<Box<
            dyn Future<Output = io::Result<(async_fs::Metadata, ffi::OsString)>> + Send
        >>,
    }
}

impl ListMetas {
    #[allow(clippy::disallowed_methods)]
    async fn new(dir_path: &AbsPath) -> io::Result<Self> {
        let _fd_permit = FileDescriptorPermit::acquire().await;

        let read_dir =
            async_fs::read_dir(dir_path).await.with_context(|| {
                format!("couldn't read directory at {dir_path}")
            })?;

        Ok(Self {
            dir_path: dir_path.to_owned(),
            get_metadata: stream::FuturesUnordered::new(),
            read_dir,
            read_dir_is_terminated: false,
            _fd_permit,
        })
    }
}

impl GetMetadata {
    fn new(dir_path: AbsPathBuf, dir_entry: async_fs::DirEntry) -> Self {
        let node_name = dir_entry.file_name();
        let entry_path = PathBuf::from(dir_path.as_str()).join(&node_name);
        let symlink_metadata = Box::pin(async move {
            let meta = async_fs::symlink_metadata(&entry_path)
                .await
                .with_context(|| {
                    format!(
                        "couldn't get metadata for entry at {}",
                        entry_path.display()
                    )
                })?;
            Ok((meta, node_name))
        });
        Self { symlink_metadata }
    }
}

impl fs::Directory for Directory {
    type EventStream = stream::Pending<fs::DirectoryEvent<RealFs>>;
    type Fs = RealFs;

    type ClearError = io::Error;
    type CreateDirectoryError = io::Error;
    type CreateFileError = io::Error;
    type CreateSymlinkError = io::Error;
    type DeleteError = io::Error;
    type ListError = io::Error;
    type MoveError = io::Error;
    type ParentError = io::Error;
    type ReadMetadataError = io::Error;

    #[inline]
    async fn create_directory(
        &self,
        directory_name: &NodeName,
    ) -> Result<Self, Self::CreateDirectoryError> {
        let path = self.path.clone().join(directory_name);
        async_fs::create_dir(&path)
            .await
            .with_context(|| format!("couldn't create directory at {path}"))?;
        let metadata = async_fs::metadata(&path).await?;
        Ok(Self { metadata, path })
    }

    #[inline]
    async fn create_file(
        &self,
        file_name: &NodeName,
    ) -> Result<File, Self::CreateFileError> {
        File::create(self.path.clone().join(file_name)).await
    }

    #[inline]
    async fn create_symlink(
        &self,
        symlink_name: &NodeName,
        target_path: &str,
    ) -> Result<Symlink, Self::CreateSymlinkError> {
        #[cfg(unix)]
        {
            let path = self.path.clone().join(symlink_name);
            async_fs::unix::symlink(target_path, &path).await.with_context(
                || format!("couldn't create symlink at {path}"),
            )?;
            let metadata = async_fs::metadata(&path).await?;
            Ok(Symlink { metadata, path })
        }
    }

    #[inline]
    async fn clear(&self) -> Result<(), Self::ClearError> {
        async {
            async_fs::remove_dir_all(self.path()).await?;
            async_fs::create_dir(self.path()).await
        }
        .await
        .with_context(|| {
            format!("couldn't clear directory at {}", self.path())
        })
    }

    #[inline]
    async fn delete(self) -> Result<(), Self::DeleteError> {
        async_fs::remove_dir_all(self.path()).await.with_context(|| {
            format!("couldn't delete directory at {}", self.path())
        })
    }

    #[allow(clippy::too_many_lines)]
    #[inline]
    async fn list_metas(&self) -> Result<ListMetas, Self::ListError> {
        ListMetas::new(self.path()).await
    }

    #[inline]
    fn meta(&self) -> Metadata {
        Metadata {
            inner: self.metadata.clone(),
            node_kind: fs::NodeKind::Directory,
            node_name: self
                .name()
                .map(|n| n.as_str().into())
                .unwrap_or_default(),
        }
    }

    #[inline]
    async fn r#move(&self, new_path: &AbsPath) -> Result<(), Self::MoveError> {
        crate::move_node(self.path(), new_path).await.with_context(|| {
            format!(
                "couldn't move directory at {} to {}",
                self.path(),
                new_path
            )
        })
    }

    #[inline]
    async fn parent(&self) -> Result<Option<Self>, Self::ParentError> {
        let Some(parent_path) = self.path().parent() else { return Ok(None) };
        let metadata =
            async_fs::metadata(parent_path).await.with_context(|| {
                format!(
                    "couldn't get metadata for directory at {parent_path}",
                )
            })?;
        Ok(Some(Self { path: parent_path.to_owned(), metadata }))
    }

    #[inline]
    fn path(&self) -> &AbsPath {
        &self.path
    }

    #[inline]
    fn watch(&self) -> Self::EventStream {
        stream::pending()
    }
}

impl AsRef<RealFs> for Directory {
    fn as_ref(&self) -> &RealFs {
        &RealFs {}
    }
}

impl Stream for ListMetas {
    type Item = io::Result<Metadata>;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            let mut read_dir_yielded_entry = false;

            if !*this.read_dir_is_terminated {
                match this.read_dir.as_mut().poll_next(ctx) {
                    Poll::Ready(Some(Ok(dir_entry))) => {
                        read_dir_yielded_entry = true;
                        this.get_metadata.push(GetMetadata::new(
                            this.dir_path.clone(),
                            dir_entry,
                        ));
                    },
                    Poll::Ready(Some(Err(error))) => {
                        return Poll::Ready(Some(Err(error)));
                    },
                    Poll::Ready(None) => {
                        *this.read_dir_is_terminated = true;
                    },
                    Poll::Pending => {},
                }
            }

            match this.get_metadata.as_mut().poll_next(ctx) {
                Poll::Ready(Some(Ok((metadata, node_name)))) => {
                    let file_type = metadata.file_type();
                    let node_kind = if file_type.is_dir() {
                        fs::NodeKind::Directory
                    } else if file_type.is_file() {
                        fs::NodeKind::File
                    } else if file_type.is_symlink() {
                        fs::NodeKind::Symlink
                    } else {
                        continue;
                    };
                    return Poll::Ready(Some(Ok(Metadata {
                        inner: metadata,
                        node_kind,
                        node_name,
                    })));
                },
                Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(Some(Err(error)));
                },
                Poll::Ready(None) => {
                    debug_assert!(this.get_metadata.is_empty());
                    return if *this.read_dir_is_terminated {
                        Poll::Ready(None)
                    } else {
                        Poll::Pending
                    };
                },
                Poll::Pending => {
                    if read_dir_yielded_entry {
                        continue;
                    } else {
                        return Poll::Pending;
                    }
                },
            }
        }
    }
}

impl FusedStream for ListMetas {
    #[inline]
    fn is_terminated(&self) -> bool {
        self.read_dir_is_terminated && self.get_metadata.is_terminated()
    }
}

impl Future for GetMetadata {
    type Output = io::Result<(async_fs::Metadata, ffi::OsString)>;

    #[inline]
    fn poll(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        self.project().symlink_metadata.poll(ctx)
    }
}
