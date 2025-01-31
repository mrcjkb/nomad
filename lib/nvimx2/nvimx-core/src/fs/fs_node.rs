use crate::fs::{self, AbsPath, FsNodeKind};

/// TODO: docs.
pub enum FsNode<Fs, Path>
where
    Fs: fs::Fs,
    Path: AsRef<AbsPath>,
{
    /// TODO: docs.
    File(Fs::File<Path>),

    /// TODO: docs.
    Directory(Fs::Directory<Path>),
}

impl<Fs, Path> FsNode<Fs, Path>
where
    Fs: fs::Fs,
    Path: AsRef<AbsPath>,
{
    /// TODO: docs.
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.kind().is_dir()
    }

    /// TODO: docs.
    #[inline]
    pub fn is_file(&self) -> bool {
        self.kind().is_file()
    }

    /// TODO: docs.
    #[inline]
    pub fn kind(&self) -> FsNodeKind {
        match self {
            Self::File(_) => FsNodeKind::File,
            Self::Directory(_) => FsNodeKind::Directory,
        }
    }
}
