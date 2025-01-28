use futures_lite::Stream;
use nvimx2::fs;

use crate::accumulate::{self, AccumulateError, Accumulator};
use crate::filter::{Filter, Filtered};

/// TODO: docs.
pub trait WalkDir<Fs: fs::Fs>: Sized {
    /// TODO: docs.
    type DirEntry: fs::DirEntry;

    /// TODO: docs.
    type ReadDir: Stream<Item = Result<Self::DirEntry, Self::DirEntryError>>;

    /// TODO: docs.
    type DirEntryError;

    /// TODO: docs.
    type ReadDirError;

    /// TODO: docs.
    fn read_dir(
        &self,
        path: &fs::AbsPath,
    ) -> impl Future<Output = Result<Self::ReadDir, Self::ReadDirError>>;

    /// TODO: docs.
    #[inline]
    fn accumulate<A>(
        &self,
        acc: &mut A,
        fs: &mut Fs,
    ) -> impl Future<Output = Result<Fs::Timestamp, AccumulateError<A, Self, Fs>>>
    where
        A: Accumulator<Fs>,
    {
        async move { accumulate::accumulate(self, acc, fs).await }
    }

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter,
    {
        Filtered::new(filter, self)
    }
}

impl<Fs: fs::Fs> WalkDir<Self> for Fs {
    type DirEntry = <Self as fs::Fs>::DirEntry;
    type ReadDir = <Self as fs::Fs>::ReadDir;
    type DirEntryError = <Self as fs::Fs>::DirEntryError;
    type ReadDirError = <Self as fs::Fs>::ReadDirError;

    async fn read_dir(
        &self,
        _path: &fs::AbsPath,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        todo!()
    }
}
