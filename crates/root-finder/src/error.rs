use core::error::Error;
use core::fmt;

use fs::{AbsPathBuf, DirEntry};

/// TODO: docs.
pub enum FindRootError<Fs: fs::Fs> {
    /// TODO: docs.
    DirEntry {
        /// TODO: docs.
        parent_path: AbsPathBuf,
        /// TODO: docs.
        err: Fs::DirEntryError,
    },

    /// TODO: docs.
    DirEntryName {
        /// TODO: docs.
        parent_path: AbsPathBuf,
        /// TODO: docs.
        err: <Fs::DirEntry as DirEntry>::NameError,
    },

    /// TODO: docs.
    DirEntryNodeKind {
        /// TODO: docs.
        entry_path: AbsPathBuf,
        /// TODO: docs.
        err: <Fs::DirEntry as DirEntry>::NodeKindError,
    },

    /// TODO: docs.
    NodeAtStartPath(Fs::NodeAtPathError),

    /// TODO: docs.
    ReadDir {
        /// TODO: docs.
        dir_path: AbsPathBuf,
        /// TODO: docs.
        err: Fs::ReadDirError,
    },

    /// TODO: docs.
    StartPathNotFound,
}

impl<Fs: fs::Fs> fmt::Debug for FindRootError<Fs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirEntry { parent_path, err } => f
                .debug_struct("DirEntry")
                .field("parent_path", parent_path)
                .field("err", err)
                .finish(),
            Self::DirEntryName { parent_path, err } => f
                .debug_struct("DirEntryName")
                .field("parent_path", parent_path)
                .field("err", err)
                .finish(),
            Self::DirEntryNodeKind { entry_path, err } => f
                .debug_struct("DirEntryKind")
                .field("entry_path", entry_path)
                .field("err", err)
                .finish(),
            Self::NodeAtStartPath(err) => {
                f.debug_struct("NodeAtStartPath").field("err", err).finish()
            },
            Self::ReadDir { dir_path, err } => f
                .debug_struct("ReadDir")
                .field("dir_path", dir_path)
                .field("err", err)
                .finish(),
            Self::StartPathNotFound => f.write_str("StartPathNotFound"),
        }
    }
}

impl<Fs: fs::Fs> fmt::Display for FindRootError<Fs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirEntry { parent_path, err } => {
                write!(
                    f,
                    "failed to get directory entry under {parent_path:?}: \
                     {err}"
                )
            },
            Self::DirEntryName { parent_path, err } => {
                write!(
                    f,
                    "failed to get name of directory entry under \
                     {parent_path:?}, {err}",
                )
            },
            Self::DirEntryNodeKind { entry_path, err } => {
                write!(
                    f,
                    "failed to get kind of directory entry at \
                     {entry_path:?}: {err}",
                )
            },
            Self::NodeAtStartPath(err) => {
                write!(f, "failed to get node at starting path: {err}")
            },
            Self::ReadDir { dir_path, err } => {
                write!(f, "failed to read directory at {dir_path:?}: {err}")
            },
            Self::StartPathNotFound => {
                write!(f, "the starting path does not exist")
            },
        }
    }
}

impl<Fs: fs::Fs> Error for FindRootError<Fs> {}
