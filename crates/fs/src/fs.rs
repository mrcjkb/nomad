use core::error::Error;
use core::future::Future;

use futures_util::Stream;

use crate::{AbsPath, DirEntry};

/// TODO: docs.
pub trait Fs {
    /// TODO: docs.
    type DirEntry: DirEntry;

    /// TODO: docs.
    type ReadDir: Stream<Item = Result<Self::DirEntry, Self::DirEntryError>>;

    /// TODO: docs.
    type DirEntryError: Error;

    /// TODO: docs.
    type ReadDirError: Error;

    /// TODO: docs.
    fn read_dir<P: AsRef<AbsPath>>(
        &self,
        dir_path: P,
    ) -> impl Future<Output = Result<Self::ReadDir, Self::ReadDirError>>;
}
