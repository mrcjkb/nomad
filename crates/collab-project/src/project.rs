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

use crate::abs_path::AbsPath;
use crate::{ProjectBuilder, binary, fs, text};

/// TODO: docs.
#[derive(Clone)]
pub struct Project {
    backlogs: Backlogs,
    contexts: Contexts,
    tree: fs::ProjectTree,
}

/// An error returned when trying to acquire a mutable reference to some
/// resource (like cursors or selections) that is not owned by the local peer.
pub struct LocalPeerIsNotOwnerError;

/// TODO: docs.
#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError;

/// TODO: docs.
pub(crate) struct State<'proj> {
    contexts: &'proj Contexts,
}

/// TODO: docs.
pub(crate) struct StateMut<'proj> {
    backlogs: &'proj mut Backlogs,
    contexts: &'proj mut Contexts,
    peer_id: PeerId,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Backlogs {
    binary: binary::BinaryEditBacklog,
    text: text::TextEditBacklog,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Contexts {
    binary: binary::BinaryCtx,
    text: text::TextCtx,
}

impl Project {
    /// TODO: docs.
    #[inline]
    pub fn builder(peer_id: PeerId) -> ProjectBuilder {
        ProjectBuilder {
            inner: fs::ProjectTreeBuilder::new(peer_id.into()),
            binary_ctx: binary::BinaryCtx::default(),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn cursor(
        &self,
        cursor_id: text::CursorId,
    ) -> Option<text::CursorRef<'_>> {
        text::CursorRef::from_id(cursor_id, self)
    }

    /// TODO: docs.
    #[inline]
    pub fn cursor_mut(
        &mut self,
        cursor_id: text::CursorId,
    ) -> Result<Option<text::CursorMut<'_>>, LocalPeerIsNotOwnerError> {
        if cursor_id.owner() == self.peer_id() {
            Ok(text::CursorMut::from_id(cursor_id, self))
        } else {
            Err(LocalPeerIsNotOwnerError)
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn cursors(&self) -> text::Cursors<'_> {
        text::Cursors::new(self)
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
    ) -> Option<fs::Directory<'_>> {
        match self.tree.directory(directory_id) {
            puff::directory::DirectoryState::Visible(directory) => {
                Some(fs::Directory::new(directory, self.state()))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn directory_mut(
        &mut self,
        directory_id: LocalDirectoryId,
    ) -> Option<fs::DirectoryMut<'_>> {
        let (state, tree) = self.state_mut();

        match tree.directory_mut(directory_id) {
            puff::directory::DirectoryMutState::Visible(directory) => {
                Some(fs::DirectoryMut::new(directory, state))
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
    pub fn file(&self, file_id: LocalFileId) -> Option<fs::File<'_>> {
        match self.tree.file(file_id) {
            puff::file::FileState::Visible(file) => {
                Some(fs::File::new(file, self.state()))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn file_mut(
        &mut self,
        file_id: LocalFileId,
    ) -> Option<fs::FileMut<'_>> {
        let (state, tree) = self.state_mut();

        match tree.file_mut(file_id) {
            puff::file::FileMutState::Visible(file) => {
                Some(fs::FileMut::new(file, state))
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_binary_edit(
        &mut self,
        binary_edit: BinaryEdit,
    ) -> Option<binary::BinaryFileMut<'_>> {
        let Some(file_id) =
            self.tree.local_file_id_of_global_id(binary_edit.file_id)
        else {
            self.backlogs.binary.insert(binary_edit);
            return None;
        };

        let mut file_state = self
            .binary_file_mut(file_id)
            .expect("BinaryEdit can only be created by a BinaryFile");

        let did_change = file_state.integrate_edit(binary_edit);

        match file_state {
            binary::BinaryStateMut::Visible(file) => {
                did_change.then_some(file)
            },
            _ => None,
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_cursor_creation(
        &mut self,
        cursor_creation: CursorCreation,
    ) -> Option<text::CursorRef<'_>> {
        let cursor = self
            .contexts
            .text
            .cursors
            .integrate_creation(cursor_creation, &self.tree)?;

        text::CursorRef::from_id(cursor.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_cursor_removal(
        &mut self,
        cursor_removal: CursorRemoval,
    ) -> Option<text::CursorId> {
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
    ) -> Option<text::CursorRef<'_>> {
        let (cursor, was_updated) =
            self.contexts.text.cursors.integrate_op(cursor_move)?;

        if !was_updated {
            return None;
        }

        text::CursorRef::from_id(cursor.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_fs_op(
        &mut self,
        op: impl fs::FsOp,
    ) -> fs::SyncActions<'_> {
        op.integrate_into(self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_peer_disconnection(
        &mut self,
        peer_id: PeerId,
    ) -> (
        impl IntoIterator<Item = text::CursorId>,
        impl IntoIterator<Item = text::SelectionId>,
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
    ) -> Option<text::SelectionRef<'_>> {
        let selection = self
            .contexts
            .text
            .selections
            .integrate_creation(selection_creation, &self.tree)?;

        text::SelectionRef::from_id(selection.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_selection_removal(
        &mut self,
        selection_removal: SelectionRemoval,
    ) -> Option<text::SelectionId> {
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
    ) -> Option<text::SelectionRef<'_>> {
        let (selection, was_updated) =
            self.contexts.text.selections.integrate_op(selection_move)?;

        if !was_updated {
            return None;
        }

        text::SelectionRef::from_id(selection.id().into(), self)
    }

    /// TODO: docs.
    #[inline]
    pub fn integrate_text_edit(
        &mut self,
        text_edit: TextEdit,
    ) -> Option<(text::TextFileMut<'_>, text::TextReplacements)> {
        let Some(file_id) =
            self.tree.local_file_id_of_global_id(text_edit.file_id)
        else {
            self.backlogs.text.insert(text_edit);
            return None;
        };

        let mut file_state = self
            .text_file_mut(file_id)
            .expect("TextEdit can only be created by a TextFile");

        let replacements = file_state.integrate_edit(text_edit);

        match file_state {
            text::TextStateMut::Visible(file) => Some((file, replacements)),
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
            backlogs: Backlogs::default(),
            contexts: Contexts::default(),
            tree: fs::ProjectTree::new((), peer_id.into()),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn node_at_path(&self, path: &AbsPath) -> Option<fs::Node<'_>> {
        match self.tree.node_at_path(path)? {
            puff::node::Node::Directory(directory) => {
                Some(fs::Node::Directory(fs::Directory::new(
                    directory,
                    self.state(),
                )))
            },
            puff::node::Node::File(file) => {
                Some(fs::Node::File(fs::File::new(file, self.state())))
            },
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn node_at_path_mut(
        &mut self,
        path: &AbsPath,
    ) -> Option<fs::NodeMut<'_>> {
        let (state, tree) = self.state_mut();

        match tree.node_at_path_mut(path)? {
            puff::node::NodeMut::Directory(directory) => {
                Some(fs::NodeMut::Directory(fs::DirectoryMut::new(
                    directory, state,
                )))
            },
            puff::node::NodeMut::File(file) => {
                Some(fs::NodeMut::File(fs::FileMut::new(file, state)))
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
    pub fn root(&self) -> fs::Directory<'_> {
        fs::Directory::new(self.tree.root(), self.state())
    }

    /// TODO: docs.
    #[inline]
    pub fn root_mut(&mut self) -> fs::DirectoryMut<'_> {
        let (state, tree) = self.state_mut();
        fs::DirectoryMut::new(tree.root_mut(), state)
    }

    /// TODO: docs.
    #[inline]
    pub fn selection(
        &self,
        selection_id: text::SelectionId,
    ) -> Option<text::SelectionRef<'_>> {
        text::SelectionRef::from_id(selection_id, self)
    }

    /// TODO: docs.
    #[inline]
    pub fn selection_mut(
        &mut self,
        selection_id: text::SelectionId,
    ) -> Result<Option<text::SelectionMut<'_>>, LocalPeerIsNotOwnerError> {
        if selection_id.owner() == self.peer_id() {
            Ok(text::SelectionMut::from_id(selection_id, self))
        } else {
            Err(LocalPeerIsNotOwnerError)
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn selections(&self) -> text::Selections<'_> {
        text::Selections::new(self)
    }

    #[inline]
    pub(crate) fn from_builder(builder: ProjectBuilder) -> Self {
        Self {
            backlogs: Backlogs::default(),
            contexts: Contexts {
                binary: builder.binary_ctx,
                text: text::TextCtx::default(),
            },
            tree: builder.inner.build(),
        }
    }

    #[inline]
    pub(crate) fn state(&self) -> State<'_> {
        State { contexts: &self.contexts }
    }

    #[inline]
    pub(crate) fn state_mut(
        &mut self,
    ) -> (StateMut<'_>, &mut fs::ProjectTree) {
        let peer_id = self.peer_id();
        let state = StateMut {
            backlogs: &mut self.backlogs,
            contexts: &mut self.contexts,
            peer_id,
        };
        (state, &mut self.tree)
    }

    #[inline]
    pub(crate) fn text_ctx(&self) -> &text::TextCtx {
        &self.contexts.text
    }

    #[inline]
    pub(crate) fn text_ctx_mut(&mut self) -> &mut text::TextCtx {
        &mut self.contexts.text
    }

    #[inline]
    pub(crate) fn tree(&self) -> &fs::ProjectTree {
        &self.tree
    }

    #[inline]
    pub(crate) fn tree_mut(&mut self) -> &mut fs::ProjectTree {
        &mut self.tree
    }

    #[inline]
    fn binary_file_mut(
        &mut self,
        file_id: LocalFileId,
    ) -> Option<binary::BinaryStateMut<'_>> {
        let (state, tree) = self.state_mut();
        binary::BinaryStateMut::new(tree.file_mut(file_id), state)
    }

    #[inline]
    fn text_file_mut(
        &mut self,
        file_id: LocalFileId,
    ) -> Option<text::TextStateMut<'_>> {
        let (state, tree) = self.state_mut();
        text::TextStateMut::new(tree.file_mut(file_id), state)
    }
}

impl<'proj> State<'proj> {
    #[inline]
    pub(crate) fn text_ctx(&self) -> &'proj text::TextCtx {
        &self.contexts.text
    }
}

impl StateMut<'_> {
    #[inline]
    pub(crate) fn as_ref(&self) -> State<'_> {
        State { contexts: self.contexts }
    }

    #[inline]
    pub(crate) fn binary_backlog_mut(
        &mut self,
    ) -> &mut binary::BinaryEditBacklog {
        &mut self.backlogs.binary
    }

    #[inline]
    pub(crate) fn binary_ctx_mut(&mut self) -> &mut binary::BinaryCtx {
        &mut self.contexts.binary
    }

    /// Returns the [`PeerId`] of the local peer.
    #[inline]
    pub(crate) fn local_id(&self) -> PeerId {
        self.peer_id
    }

    #[inline]
    pub(crate) fn reborrow(&mut self) -> StateMut<'_> {
        StateMut {
            backlogs: self.backlogs,
            contexts: self.contexts,
            peer_id: self.peer_id,
        }
    }

    #[inline]
    pub(crate) fn text_backlog_mut(&mut self) -> &mut text::TextEditBacklog {
        &mut self.backlogs.text
    }

    #[inline]
    pub(crate) fn text_ctx_mut(&mut self) -> &mut text::TextCtx {
        &mut self.contexts.text
    }
}

impl Copy for State<'_> {}

impl Clone for State<'_> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
