use core::error::Error;

use futures_util::Stream;

use crate::fs::{self, AbsPath, Fs, FsNode, NodeDeletion, NodeMove, NodeName};

/// TODO: docs.
pub trait Symlink: Send {
    /// TODO: docs.
    type EventStream: Stream<Item = SymlinkEvent<Self::Fs>> + Unpin;

    /// TODO: docs.
    type Fs: Fs;

    /// TODO: docs.
    type DeleteError: Error + Send;

    /// TODO: docs.
    type FollowError: Error + Send;

    /// TODO: docs.
    type MetadataError: Error + Send;

    /// TODO: docs.
    type ReadError: Error + Send;

    /// TODO: docs.
    fn delete(self) -> impl Future<Output = Result<(), Self::DeleteError>>;

    /// TODO: docs.
    fn follow(
        &self,
    ) -> impl Future<Output = Result<Option<FsNode<Self::Fs>>, Self::FollowError>>;

    /// TODO: docs.
    fn follow_recursively(
        &self,
    ) -> impl Future<Output = Result<Option<FsNode<Self::Fs>>, Self::FollowError>>;

    /// TODO: docs.
    fn id(&self) -> <Self::Fs as Fs>::NodeId;

    /// TODO: docs.
    fn meta(
        &self,
    ) -> impl Future<Output = Result<<Self::Fs as Fs>::Metadata, Self::MetadataError>>;

    /// TODO: docs.
    fn name(&self) -> &NodeName {
        self.path().node_name().expect("path is not root")
    }

    /// TODO: docs.
    fn path(&self) -> &AbsPath;

    /// TODO: docs.
    fn read_path(
        &self,
    ) -> impl Future<Output = Result<String, Self::ReadError>> + Send;

    /// TODO: docs.
    fn watch(&self) -> Self::EventStream;
}

/// TODO: docs.
pub enum SymlinkEvent<Fs: fs::Fs> {
    /// TODO: docs.
    Deletion(NodeDeletion<Fs>),

    /// TODO: docs.
    Move(NodeMove<Fs>),
}
