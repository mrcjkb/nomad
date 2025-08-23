use core::ops::{Deref, DerefMut};
use std::io;

use abs_path::AbsPath;
use fs::Fs;

use crate::{Directory, File, RealFs};

/// TODO: docs.
pub struct TempDirectory {
    pub(crate) inner: Directory,

    /// We need to keep the inner handle around so that the directory is
    /// deleted when `Self` is dropped.
    pub(crate) _handle: tempfile::TempDir,
}

/// TODO: docs.
pub struct TempFile {
    pub(crate) inner: File,

    /// We need to keep the inner handle around so that the file is
    /// deleted when `Self` is dropped.
    pub(crate) _handle: tempfile::NamedTempFile,
}

impl RealFs {
    /// Creates a new temporary directory that will be deleted when the
    /// [`TempDirectory`] is dropped.
    pub async fn tempdir(&self) -> io::Result<TempDirectory> {
        let tempdir = tempfile::TempDir::new()?;

        let inner = self
            .node_at_path(abs_path(tempdir.path())?)
            .await?
            .expect("just created the directory")
            .unwrap_directory();

        Ok(crate::TempDirectory { inner, _handle: tempdir })
    }

    /// Creates a new temporary file that will be deleted when the
    /// [`TempFile`] is dropped.
    pub async fn tempfile(&self) -> io::Result<TempFile> {
        let tempfile = tempfile::NamedTempFile::new()?;

        let inner = self
            .node_at_path(abs_path(tempfile.path())?)
            .await?
            .expect("just created the file")
            .unwrap_file();

        Ok(crate::TempFile { inner, _handle: tempfile })
    }
}

fn abs_path(path: &std::path::Path) -> io::Result<&AbsPath> {
    <&AbsPath>::try_from(path).map_err(|err| {
        let err_msg = match err {
            abs_path::AbsPathFromPathError::NotAbsolute => {
                format!("{:?} is not absolute", path)
            },
            abs_path::AbsPathFromPathError::NotUtf8 => {
                format!("{:?} is not valid UTF-8", path)
            },
        };

        io::Error::new(io::ErrorKind::InvalidFilename, err_msg)
    })
}

impl Deref for TempDirectory {
    type Target = Directory;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TempDirectory {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for TempFile {
    type Target = File;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TempFile {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
