use crate::fs::{self, AbsPathBuf};

/// TODO: docs.
#[derive(Debug)]
pub struct FsEvent<Fs: fs::Fs> {
    /// TODO: docs.
    pub kind: FsEventKind,

    /// TODO: docs.
    pub path: AbsPathBuf,

    /// TODO: docs.
    pub timestamp: Fs::Timestamp,
}

/// TODO: docs.
#[derive(Debug, Clone)]
pub enum FsEventKind {
    /// TODO: docs.
    CreatedDir,
}

impl<Fs> Clone for FsEvent<Fs>
where
    Fs: fs::Fs,
    Fs::Timestamp: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            path: self.path.clone(),
            timestamp: self.timestamp.clone(),
        }
    }
}
