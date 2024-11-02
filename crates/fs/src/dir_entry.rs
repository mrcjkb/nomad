use alloc::borrow::Cow;
use core::error::Error;
use core::future::Future;

use e31e::fs::FsNodeName;

use crate::FsNodeKind;

/// TODO: docs.
pub trait DirEntry {
    /// TODO: docs.
    type NameError: Error;

    /// TODO: docs.
    type NodeKindError: Error;

    /// TODO: docs.
    fn name(
        &self,
    ) -> impl Future<Output = Result<Cow<'_, FsNodeName>, Self::NameError>>;

    /// TODO: docs.
    fn node_kind(
        &self,
    ) -> impl Future<Output = Result<FsNodeKind, Self::NodeKindError>>;
}
