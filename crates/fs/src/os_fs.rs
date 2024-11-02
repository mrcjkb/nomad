//! TODO: docs.

use alloc::borrow::Cow;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::io;

use futures_util::Stream;

use crate::{AbsPath, DirEntry, Fs, FsNodeKind, FsNodeName};

/// TODO: docs.
pub struct OsFs {}

/// TODO: docs.
pub struct OsReadDir {}

/// TODO: docs.
pub struct OsDirEntry {}

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
    type NameError = io::Error;
    type NodeKindError = io::Error;

    async fn name(&self) -> Result<Cow<'_, FsNodeName>, Self::NameError> {
        todo!();
    }

    async fn node_kind(&self) -> Result<FsNodeKind, Self::NodeKindError> {
        todo!();
    }
}
