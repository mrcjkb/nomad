use core::error::Error;

use abs_path::AbsPathBuf;
use futures_lite::Stream;

use crate::fs::{self, AbsPath, Fs, NodeName};

/// TODO: docs.
pub trait Directory: Sized {
    /// TODO: docs.
    type EventStream: Stream<Item = DirectoryEvent<Self::Fs>> + Unpin;

    /// TODO: docs.
    type Fs: Fs;

    /// TODO: docs.
    type CreateDirectoryError: Error;

    /// TODO: docs.
    type CreateFileError: Error;

    /// TODO: docs.
    type ClearError: Error;

    /// TODO: docs.
    type DeleteError: Error;

    /// TODO: docs.
    type MetadataError: Error;

    /// TODO: docs.
    type ReadEntryError: Error;

    /// TODO: docs.
    type ReadError: Error;

    /// TODO: docs.
    fn create_directory(
        &self,
        directory_name: &NodeName,
    ) -> impl Future<Output = Result<Self, Self::CreateDirectoryError>>;

    /// TODO: docs.
    fn create_file(
        &self,
        file_name: &NodeName,
    ) -> impl Future<Output = Result<<Self::Fs as Fs>::File, Self::CreateFileError>>;

    /// TODO: docs.
    fn clear(&self) -> impl Future<Output = Result<(), Self::ClearError>>;

    /// TODO: docs.
    fn delete(self) -> impl Future<Output = Result<(), Self::DeleteError>>;

    /// TODO: docs.
    fn meta(
        &self,
    ) -> impl Future<Output = Result<<Self::Fs as Fs>::Metadata, Self::MetadataError>>;

    /// TODO: docs.
    #[inline]
    fn name(&self) -> Option<&NodeName> {
        self.path().node_name()
    }

    /// TODO: docs.
    fn parent(
        &self,
    ) -> impl Future<Output = Option<<Self::Fs as Fs>::Directory>>;

    /// TODO: docs.
    fn path(&self) -> &AbsPath;

    /// TODO: docs.
    fn read(
        &self,
    ) -> impl Future<
        Output = Result<
            impl Stream<
                Item = Result<
                    <Self::Fs as Fs>::Metadata,
                    Self::ReadEntryError,
                >,
            > + use<Self>,
            Self::ReadError,
        >,
    >;

    /// TODO: docs.
    fn watch(&self) -> impl Future<Output = Self::EventStream>;
}

/// TODO: docs.
pub enum DirectoryEvent<Fs: fs::Fs> {
    /// TODO: docs.
    Creation(NodeCreation<Fs>),

    /// TODO: docs.
    Deletion(DirectoryDeletion<Fs>),

    /// TODO: docs.
    Move(DirectoryMove<Fs>),
}

/// TODO: docs.
pub struct DirectoryDeletion<Fs: fs::Fs> {
    /// The node ID of the directory.
    pub dir_id: Fs::NodeId,

    /// The path to the directory at the time of its deletion.
    pub dir_path: AbsPathBuf,

    /// TODO: docs.
    pub deletion_root_id: Fs::NodeId,
}

/// TODO: docs.
pub struct DirectoryMove<Fs: fs::Fs> {
    /// The node ID of the directory.
    pub dir_id: Fs::NodeId,

    /// The path to the directory before it was moved.
    pub old_path: AbsPathBuf,

    /// The path to the directory after it was moved.
    pub new_path: AbsPathBuf,

    /// TODO: docs.
    pub move_root_id: Fs::NodeId,
}

/// TODO: docs.
pub struct NodeCreation<Fs: fs::Fs> {
    /// TODO: docs.
    pub node_id: Fs::NodeId,

    /// TODO: docs.
    pub node_path: AbsPathBuf,

    /// TODO: docs.
    pub parent_id: Fs::NodeId,
}
