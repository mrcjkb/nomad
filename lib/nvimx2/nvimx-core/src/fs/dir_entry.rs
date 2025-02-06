use core::error::Error;
use core::future::Future;
use std::borrow::Cow;

use crate::fs::{self, FsNodeKind, FsNodeName, Metadata};

/// TODO: docs.
pub trait DirEntry<Fs: fs::Fs> {
    /// TODO: docs.
    type MetadataError: Error;

    /// TODO: docs.
    type NameError: Error;

    /// TODO: docs.
    type NodeKindError: Error;

    /// TODO: docs.
    fn metadata(
        &self,
    ) -> impl Future<Output = Result<Metadata<Fs>, Self::MetadataError>>;

    /// TODO: docs.
    fn name(
        &self,
    ) -> impl Future<Output = Result<Cow<'_, FsNodeName>, Self::NameError>>;

    /// TODO: docs.
    fn node_kind(
        &self,
    ) -> impl Future<Output = Result<Option<FsNodeKind>, Self::NodeKindError>>;

    /// TODO: docs.
    fn is_directory(
        &self,
    ) -> impl Future<Output = Result<bool, Self::NodeKindError>> {
        async {
            self.node_kind()
                .await
                .map(|maybe_kind| maybe_kind.is_some_and(|kind| kind.is_dir()))
        }
    }

    /// TODO: docs.
    fn is_file(
        &self,
    ) -> impl Future<Output = Result<bool, Self::NodeKindError>> {
        async {
            self.node_kind().await.map(|maybe_kind| {
                maybe_kind.is_some_and(|kind| kind.is_file())
            })
        }
    }
}
