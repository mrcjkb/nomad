use core::convert::Infallible;

use ed::ByteOffset;
use ed::fs::{self, FsNodeKind, NodeName, NodeNameBuf};

use crate::{WalkDir, WalkErrorKind};

/// TODO: docs.
pub struct DirEntry<'a, W: WalkDir> {
    inner: W::DirEntry,
    name: NodeNameBuf,
    node_kind: FsNodeKind,
    parent_path: &'a fs::AbsPath,
}

impl<'a, W: WalkDir> DirEntry<'a, W> {
    /// TODO: docs.
    pub fn inner(&self) -> &W::DirEntry {
        &self.inner
    }

    /// TODO: docs.
    pub fn inner_mut(&mut self) -> &mut W::DirEntry {
        &mut self.inner
    }

    /// TODO: docs.
    pub fn into_inner(self) -> W::DirEntry {
        self.inner
    }

    /// TODO: docs.
    #[allow(clippy::same_name_method)]
    pub fn name(&self) -> &NodeName {
        &self.name
    }

    /// TODO: docs.
    #[allow(clippy::same_name_method)]
    pub fn node_kind(&self) -> FsNodeKind {
        self.node_kind
    }

    /// TODO: docs.
    pub fn parent_path(&self) -> &'a fs::AbsPath {
        self.parent_path
    }

    /// TODO: docs.
    pub fn path(&self) -> fs::AbsPathBuf {
        let mut path = self.parent_path.to_owned();
        path.push(self.name());
        path
    }

    /// TODO: docs.
    pub(crate) async fn new(
        parent_path: &'a fs::AbsPath,
        inner: W::DirEntry,
    ) -> Result<Self, WalkErrorKind<W>> {
        use fs::Metadata;
        let node_kind = inner
            .node_kind()
            .await
            .map_err(WalkErrorKind::DirEntryNodeKind)?;
        let name = inner.name().await.map_err(WalkErrorKind::DirEntryName)?;
        Ok(Self { inner, name, node_kind, parent_path })
    }
}

impl<W: WalkDir> fs::Metadata for DirEntry<'_, W> {
    type Timestamp = <W::Fs as fs::Fs>::Timestamp;
    type NameError = Infallible;
    type NodeKindError = Infallible;

    fn created_at(&self) -> Option<Self::Timestamp> {
        self.inner.created_at()
    }

    fn last_modified_at(&self) -> Option<Self::Timestamp> {
        self.inner.last_modified_at()
    }

    fn byte_len(&self) -> ByteOffset {
        self.inner.byte_len()
    }

    async fn name(&self) -> Result<NodeNameBuf, Self::NameError> {
        Ok(self.name.clone())
    }

    async fn node_kind(&self) -> Result<FsNodeKind, Self::NodeKindError> {
        Ok(self.node_kind)
    }
}
