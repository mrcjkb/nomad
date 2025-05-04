use abs_path::{AbsPath, NodeName};
use ed::fs::{Directory, Fs, os};
use futures_util::Stream;

/// TODO: docs.
pub struct TempDir {
    /// We need to keep the inner `TempDir` around so that the directory can
    /// be deleted when it is dropped.
    _inner: tempdir::TempDir,
    os_dir: os::OsDirectory,
}

impl TempDir {
    pub(crate) fn new(
        inner: tempdir::TempDir,
        os_dir: os::OsDirectory,
    ) -> Self {
        Self { _inner: inner, os_dir }
    }
}

impl Directory for TempDir {
    type EventStream = <os::OsDirectory as Directory>::EventStream;
    type Fs = <os::OsDirectory as Directory>::Fs;
    type CreateDirectoryError =
        <os::OsDirectory as Directory>::CreateDirectoryError;
    type CreateFileError = <os::OsDirectory as Directory>::CreateFileError;
    type CreateSymlinkError =
        <os::OsDirectory as Directory>::CreateSymlinkError;
    type ClearError = <os::OsDirectory as Directory>::ClearError;
    type DeleteError = <os::OsDirectory as Directory>::DeleteError;
    type ParentError = <os::OsDirectory as Directory>::ParentError;
    type ReadEntryError = <os::OsDirectory as Directory>::ReadEntryError;
    type ReadError = <os::OsDirectory as Directory>::ReadError;

    async fn create_directory(
        &self,
        directory_name: &NodeName,
    ) -> Result<<Self::Fs as Fs>::Directory, Self::CreateDirectoryError> {
        <os::OsDirectory as Directory>::create_directory(
            &self.os_dir,
            directory_name,
        )
        .await
    }

    async fn create_file(
        &self,
        file_name: &NodeName,
    ) -> Result<<Self::Fs as Fs>::File, Self::CreateFileError> {
        <os::OsDirectory as Directory>::create_file(&self.os_dir, file_name)
            .await
    }

    async fn create_symlink(
        &self,
        symlink_name: &NodeName,
        target_path: &str,
    ) -> Result<<Self::Fs as Fs>::Symlink, Self::CreateSymlinkError> {
        <os::OsDirectory as Directory>::create_symlink(
            &self.os_dir,
            symlink_name,
            target_path,
        )
        .await
    }

    async fn clear(&self) -> Result<(), Self::ClearError> {
        <os::OsDirectory as Directory>::clear(&self.os_dir).await
    }

    async fn delete(self) -> Result<(), Self::DeleteError> {
        <os::OsDirectory as Directory>::delete(self.os_dir).await
    }

    fn meta(&self) -> <Self::Fs as Fs>::Metadata {
        <os::OsDirectory as Directory>::meta(&self.os_dir)
    }

    async fn parent(
        &self,
    ) -> Result<Option<<Self::Fs as Fs>::Directory>, Self::ParentError> {
        <os::OsDirectory as Directory>::parent(&self.os_dir).await
    }

    fn path(&self) -> &AbsPath {
        <os::OsDirectory as Directory>::path(&self.os_dir)
    }

    async fn read(
        &self,
    ) -> Result<
        impl Stream<
            Item = Result<<Self::Fs as Fs>::Metadata, Self::ReadEntryError>,
        > + Send
        + use<>,
        Self::ReadError,
    > {
        <os::OsDirectory as Directory>::read(&self.os_dir).await
    }

    fn watch(&self) -> Self::EventStream {
        <os::OsDirectory as Directory>::watch(&self.os_dir)
    }
}
