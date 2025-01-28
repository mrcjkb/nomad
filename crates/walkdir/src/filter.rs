use nvimx2::fs;

use crate::WalkDir;

/// TODO: docs.
pub trait Filter {}

/// TODO: docs.
pub struct Filtered<F, W> {
    _filter: F,
    _walker: W,
}

impl<F, W> Filtered<F, W> {
    /// TODO: docs.
    #[inline]
    pub(crate) fn new(filter: F, walker: W) -> Self {
        Self { _filter: filter, _walker: walker }
    }
}

impl<F, W, Fs> WalkDir<Fs> for Filtered<F, W>
where
    F: Filter,
    W: WalkDir<Fs>,
    Fs: fs::Fs,
{
    type DirEntry = W::DirEntry;
    type ReadDir = W::ReadDir;
    type DirEntryError = W::DirEntryError;
    type ReadDirError = W::ReadDirError;

    async fn read_dir(
        &self,
        _path: &fs::AbsPath,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        todo!()
    }
}
