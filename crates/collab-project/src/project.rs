use std::io::Write;

use collab_types::annotation::AnnotationDeletion;
use collab_types::binary::BinaryEdit;
use collab_types::text::{
    CursorCreation,
    CursorMove,
    CursorRemoval,
    SelectionCreation,
    SelectionMove,
    SelectionRemoval,
    TextEdit,
};
use collab_types::{PeerId, puff};
use puff::directory::{GlobalDirectoryId, LocalDirectoryId};
use puff::file::{GlobalFileId, LocalFileId};
use smallvec::SmallVec;

use crate::ProjectBuilder;
use crate::abs_path::AbsPath;
use crate::binary::{
    BinaryCtx,
    BinaryCtxState,
    BinaryEditBacklog,
    BinaryFileMut,
    BinaryStateMut,
};
use crate::fs::{
    Directory,
    DirectoryMut,
    File,
    FileMut,
    FsOp,
    Node,
    NodeMut,
    ProjectTree,
    SyncActions,
};
use crate::text::{
    CursorId,
    CursorMut,
    CursorRef,
    Cursors,
    SelectionId,
    SelectionMut,
    SelectionRef,
    Selections,
    TextCtx,
    TextCtxState,
    TextEditBacklog,
    TextFileMut,
    TextReplacements,
    TextStateMut,
};

/// TODO: docs.
#[derive(Clone)]
pub struct Project {
    pub(crate) backlog: Backlogs,
    pub(crate) contexts: Contexts,
    pub(crate) tree: ProjectTree,
}

/// An error returned when trying to acquire a mutable reference to some
/// resource (like cursors or selections) that is not owned by the local peer.
pub struct LocalPeerIsNotOwnerError;

/// TODO: docs.
#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError;

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct Backlogs {
    pub(crate) binary: BinaryEditBacklog,
    pub(crate) text: TextEditBacklog,
}

#[derive(Clone)]
pub(crate) struct Contexts {
    pub(crate) binary: BinaryCtx,
    pub(crate) text: TextCtx,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct ContextsState {
    pub(crate) binary: BinaryCtxState,
    pub(crate) text: TextCtxState,
}

impl Project {
    /// TODO: docs.
    #[inline]
    pub fn builder(peer_id: PeerId) -> ProjectBuilder {
        ProjectBuilder::new(peer_id)
    }

    /// TODO: docs.
    #[inline]
    pub fn cursor(&self, cursor_id: CursorId) -> Option<CursorRef<'_>> {
        CursorRef::from_id(cursor_id, self)
    }

    /// TODO: docs.
    #[inline]
    pub fn cursor_mut(
        &mut self,
        cursor_id: CursorId,
    ) -> Result<Option<CursorMut<'_>>, LocalPeerIsNotOwnerError> {
        if cursor_id.owner() == self.peer_id() {
            Ok(CursorMut::from_id(cursor_id, self))
        } else {
            Err(LocalPeerIsNotOwnerError)
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn cursors(&self) -> Cursors<'_> {
        Cursors::new(self)
    }

    /// TODO: docs.
    #[inline]
    pub fn decode(
        _encoded_buf: &[u8],
        _local_id: PeerId,
    ) -> Result<Self, DecodeError> {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn directory(
        &self,
        directory_id: LocalDirectoryId,
    ) -> Option<Directory<'_>> {
        match self.tree.directory(directory_id) {
            puff::directory::DirectoryState::Visible(directory) => {
                Some(Directory::new(directory, &self.contexts))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn directory_mut(
        &mut self,
        directory_id: LocalDirectoryId,
    ) -> Option<DirectoryMut<'_>> {
        match self.tree.directory_mut(directory_id) {
            puff::directory::DirectoryMutState::Visible(directory) => {
                Some(DirectoryMut::new(directory, &mut self.contexts))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_into(&mut buf);
        buf
    }

    /// TODO: docs.
    #[inline]
    pub fn encode_into(&self, _buf: &mut impl Write) {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn file(&self, file_id: LocalFileId) -> Option<File<'_>> {
        match self.tree.file(file_id) {
            puff::file::FileState::Visible(file) => {
                Some(File::new(file, &self.contexts))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn file_mut(&mut self, file_id: LocalFileId) -> Option<FileMut<'_>> {
        match self.tree.file_mut(file_id) {
            puff::file::FileMutState::Visible(file) => {
                Some(FileMut::new(file, &mut self.contexts))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_binary_edit(
        &mut self,
        binary_edit: BinaryEdit,
    ) -> Option<BinaryFileMut<'_>> {
        let Some(file_id) =
            self.tree.local_file_id_of_global_id(binary_edit.file_id)
        else {
            self.backlog.binary.insert(binary_edit);
            return None;
        };

        let mut file_state = self
            .binary_file_mut(file_id)
            .expect("BinaryEdit can only be created by a BinaryFile");

        let did_change = file_state.integrate_edit(binary_edit);

        match file_state {
            BinaryStateMut::Visible(file) => did_change.then_some(file),
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_cursor_creation(
        &mut self,
        cursor_creation: CursorCreation,
    ) -> Option<CursorRef<'_>> {
        let cursor = self
            .contexts
            .text
            .cursors
            .integrate_creation(cursor_creation, &self.tree)?;

        CursorRef::from_id(cursor.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_cursor_removal(
        &mut self,
        cursor_removal: CursorRemoval,
    ) -> Option<CursorId> {
        self.contexts
            .text
            .cursors
            .integrate_deletion(cursor_removal)
            .map(|(annotation_id, _)| annotation_id.into())
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_cursor_move(
        &mut self,
        cursor_move: CursorMove,
    ) -> Option<CursorRef<'_>> {
        let (cursor, was_updated) =
            self.contexts.text.cursors.integrate_op(cursor_move)?;

        if !was_updated {
            return None;
        }

        CursorRef::from_id(cursor.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_fs_op(&mut self, op: impl FsOp) -> SyncActions<'_> {
        FsOp::integrate_into(op, self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_peer_disconnection(
        &mut self,
        peer_id: PeerId,
    ) -> (
        impl IntoIterator<Item = CursorId>,
        impl IntoIterator<Item = SelectionId>,
    ) {
        let deleted_cursors = self
            .cursors()
            .filter_map(|cursor| {
                (cursor.owner() == peer_id).then_some(cursor.id())
            })
            .collect::<SmallVec<[_; 2]>>();

        let deleted_selections = self
            .selections()
            .filter_map(|selection| {
                (selection.owner() == peer_id).then_some(selection.id())
            })
            .collect::<SmallVec<[_; 2]>>();

        for &cursor_id in &deleted_cursors {
            let deletion =
                AnnotationDeletion { annotation_id: cursor_id.into() };
            self.contexts.text.cursors.integrate_deletion(deletion);
        }

        for &selection_id in &deleted_selections {
            let deletion =
                AnnotationDeletion { annotation_id: selection_id.into() };
            self.contexts.text.selections.integrate_deletion(deletion);
        }

        (deleted_cursors, deleted_selections)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_selection_creation(
        &mut self,
        selection_creation: SelectionCreation,
    ) -> Option<SelectionRef<'_>> {
        let selection = self
            .contexts
            .text
            .selections
            .integrate_creation(selection_creation, &self.tree)?;

        SelectionRef::from_id(selection.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_selection_removal(
        &mut self,
        selection_removal: SelectionRemoval,
    ) -> Option<SelectionId> {
        self.contexts
            .text
            .selections
            .integrate_deletion(selection_removal)
            .map(|(annotation_id, _)| annotation_id.into())
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_selection_move(
        &mut self,
        selection_move: SelectionMove,
    ) -> Option<SelectionRef<'_>> {
        let (selection, was_updated) =
            self.contexts.text.selections.integrate_op(selection_move)?;

        if !was_updated {
            return None;
        }

        SelectionRef::from_id(selection.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_text_edit(
        &mut self,
        text_edit: TextEdit,
    ) -> Option<(TextFileMut<'_>, TextReplacements)> {
        let Some(file_id) =
            self.tree.local_file_id_of_global_id(text_edit.file_id)
        else {
            self.backlog.text.insert(text_edit);
            return None;
        };

        let mut file_state = self
            .text_file_mut(file_id)
            .expect("TextEdit can only be created by a TextFile");

        let replacements = file_state.integrate_edit(text_edit);

        match file_state {
            TextStateMut::Visible(file) => Some((file, replacements)),
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn local_directory_of_global(
        &self,
        global_id: GlobalDirectoryId,
    ) -> Option<LocalDirectoryId> {
        self.tree.local_directory_id_of_global_id(global_id)
    }

    /// TODO: docs.
    #[inline]
    pub fn local_file_of_global(
        &self,
        global_id: GlobalFileId,
    ) -> Option<LocalFileId> {
        self.tree.local_file_id_of_global_id(global_id)
    }

    /// TODO: docs.
    #[inline]
    pub fn new(peer_id: PeerId) -> Self {
        Self {
            backlog: Backlogs::default(),
            contexts: Contexts::new(peer_id),
            tree: ProjectTree::new((), peer_id.into()),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn node_at_path(&self, path: &AbsPath) -> Option<Node<'_>> {
        match self.tree.node_at_path(path)? {
            puff::node::Node::Directory(directory) => Some(Node::Directory(
                Directory::new(directory, &self.contexts),
            )),
            puff::node::Node::File(file) => {
                Some(Node::File(File::new(file, &self.contexts)))
            },
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn node_at_path_mut(&mut self, path: &AbsPath) -> Option<NodeMut<'_>> {
        match self.tree.node_at_path_mut(path)? {
            puff::node::NodeMut::Directory(directory) => {
                Some(NodeMut::Directory(DirectoryMut::new(
                    directory,
                    &mut self.contexts,
                )))
            },
            puff::node::NodeMut::File(file) => {
                Some(NodeMut::File(FileMut::new(file, &mut self.contexts)))
            },
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn peer_id(&self) -> PeerId {
        self.tree.peer_id().into()
    }

    /// TODO: docs.
    #[inline]
    pub fn root(&self) -> Directory<'_> {
        Directory::new(self.tree.root(), &self.contexts)
    }

    /// TODO: docs.
    #[inline]
    pub fn root_mut(&mut self) -> DirectoryMut<'_> {
        DirectoryMut::new(self.tree.root_mut(), &mut self.contexts)
    }

    /// TODO: docs.
    #[inline]
    pub fn selection(
        &self,
        selection_id: SelectionId,
    ) -> Option<SelectionRef<'_>> {
        SelectionRef::from_id(selection_id, self)
    }

    /// TODO: docs.
    #[inline]
    pub fn selection_mut(
        &mut self,
        selection_id: SelectionId,
    ) -> Result<Option<SelectionMut<'_>>, LocalPeerIsNotOwnerError> {
        if selection_id.owner() == self.peer_id() {
            Ok(SelectionMut::from_id(selection_id, self))
        } else {
            Err(LocalPeerIsNotOwnerError)
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn selections(&self) -> Selections<'_> {
        Selections::new(self)
    }

    #[inline]
    fn binary_file_mut(
        &mut self,
        file_id: LocalFileId,
    ) -> Option<BinaryStateMut<'_>> {
        BinaryStateMut::new(self.tree.file_mut(file_id), &mut self.contexts)
    }

    #[inline]
    fn text_file_mut(
        &mut self,
        file_id: LocalFileId,
    ) -> Option<TextStateMut<'_>> {
        TextStateMut::new(self.tree.file_mut(file_id), &mut self.contexts)
    }
}

impl Contexts {
    #[inline]
    pub(crate) fn new(local_id: PeerId) -> Self {
        Self { binary: BinaryCtx::new(local_id), text: TextCtx::new(local_id) }
    }
}
