//! TODO: docs.

use alloc::borrow::Cow;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::ffi::OsString;
use std::io;

use futures_util::Stream;

use crate::{
    AbsPath,
    DirEntry,
    Fs,
    FsNodeKind,
    FsNodeName,
    InvalidFsNodeNameError,
};

/// TODO: docs.
pub struct OsFs {}

/// TODO: docs.
pub struct OsReadDir {}

/// TODO: docs.
pub struct OsDirEntry {
    inner: async_fs::DirEntry,
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
    type ReadDir = OsReadDir;
    type DirEntryError = io::Error;
    type ReadDirError = io::Error;

    async fn read_dir<P: AsRef<AbsPath>>(
        &self,
        dir_path: P,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        todo!();
    }
}

impl Stream for OsReadDir {
    type Item = Result<OsDirEntry, io::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        todo!();
    }
}

impl DirEntry for OsDirEntry {
    type NameError = OsNameError;
    type NodeKindError = io::Error;

    async fn name(&self) -> Result<Cow<'_, FsNodeName>, Self::NameError> {
        let os_name = self.inner.file_name();
        let fs_name: &FsNodeName = os_name
            .to_str()
            .ok_or_else(|| OsNameError::NotUtf8(os_name.clone()))?
            .try_into()?;
        Ok(Cow::Owned(fs_name.to_owned()))
    }

    async fn node_kind(&self) -> Result<FsNodeKind, Self::NodeKindError> {
        self.inner.file_type().await.map(Into::into)
    }
}
