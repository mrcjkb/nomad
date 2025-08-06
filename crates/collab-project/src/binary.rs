//! TODO: docs.

use std::collections::hash_map::Entry;

use bytes::Bytes;
use collab_types::binary::{BinaryEdit, BinaryEditTimestamp};
use collab_types::lamport::LamportClock;
use collab_types::{PeerId, bytes, puff};
use fxhash::FxHashMap;
use puff::file::{GlobalFileId, LocalFileId};
use puff::node::{Backlogged, Deleted, Editable, IsVisible};

use crate::abs_path::AbsPathBuf;
use crate::fs::{
    FileContents,
    FileMut,
    PuffFile,
    PuffFileMut,
    PuffFileStateMut,
};
use crate::project::{State, StateMut};

/// TODO: docs.
pub struct BinaryFile<'proj, S = Editable> {
    inner: PuffFile<'proj, S>,
    state: State<'proj>,
}

/// TODO: docs.
pub struct BinaryFileMut<'proj, S = Editable> {
    inner: PuffFileMut<'proj, S>,
    state: StateMut<'proj>,
}

/// TODO: docs.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct BinaryContents {
    inner: Bytes,
    set_at: BinaryEditTimestamp,
}

#[derive(Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct BinaryCtx {
    edit_clock: LamportClock,
}

/// TODO: docs.
#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct BinaryEditBacklog {
    /// Map from a backlogged file's global ID to the most recent contents
    /// received for that file.
    edits: FxHashMap<GlobalFileId, BinaryEdit>,
}

/// The state of a binary file.
pub(crate) enum BinaryStateMut<'a> {
    Visible(BinaryFileMut<'a, Editable>),
    Backlogged(BinaryFileMut<'a, Backlogged>),
    Deleted(BinaryFileMut<'a, Deleted>),
}

impl<'a, S> BinaryFile<'a, S> {
    /// Returns the binary file's contents.
    #[inline]
    pub fn contents(&self) -> &'a [u8] {
        match self.inner.metadata() {
            FileContents::Binary(binary_contents) => &binary_contents.inner,
            _ => unreachable!(),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn id(&self) -> LocalFileId {
        self.inner.local_id()
    }

    #[inline]
    pub(crate) fn inner(&self) -> PuffFile<'a, S> {
        self.inner
    }

    #[track_caller]
    #[inline]
    pub(crate) fn new(inner: PuffFile<'a, S>, state: State<'a>) -> Self {
        debug_assert!(inner.metadata().is_binary());
        Self { inner, state }
    }

    #[inline]
    pub(crate) fn state(&self) -> State<'a> {
        self.state
    }
}

impl<'a, S: IsVisible> BinaryFile<'a, S> {
    /// TODO: docs.
    #[inline]
    pub fn path(&self) -> AbsPathBuf {
        self.inner.path()
    }
}

impl<'a, S> BinaryFileMut<'a, S> {
    /// TODO: docs.
    #[inline]
    pub fn as_file(&self) -> BinaryFile<'_, S> {
        BinaryFile { inner: self.inner.as_file(), state: self.state.as_ref() }
    }

    #[inline]
    pub(crate) fn inner_mut(&mut self) -> &mut PuffFileMut<'a, S> {
        &mut self.inner
    }

    #[inline]
    pub(crate) fn integrate_edit(&mut self, edit: BinaryEdit) -> bool {
        debug_assert_eq!(edit.file_id, self.inner.global_id());

        match self.inner.metadata_mut() {
            FileContents::Binary(contents) => {
                contents.integrate_edit(edit, self.state.binary_ctx_mut())
            },
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn into_inner(self) -> PuffFileMut<'a, S> {
        self.inner
    }

    #[track_caller]
    #[inline]
    pub(crate) fn new(inner: PuffFileMut<'a, S>, state: StateMut<'a>) -> Self {
        debug_assert!(inner.metadata().is_binary());
        Self { inner, state }
    }

    #[inline]
    fn contents_mut(&mut self) -> &mut BinaryContents {
        match self.inner.metadata_mut() {
            FileContents::Binary(binary_contents) => binary_contents,
            _ => unreachable!(),
        }
    }
}

impl<'a> BinaryFileMut<'a, Editable> {
    /// Replaces the file's contents with the given contents.
    #[inline]
    pub fn replace(&mut self, new_contents: impl Into<Bytes>) -> BinaryEdit {
        let new_contents = new_contents.into();
        let contents = BinaryContents::new_local(
            self.state.local_id(),
            new_contents.clone(),
            self.state.binary_ctx_mut(),
        );
        let old_contents = self.contents_mut();
        debug_assert!(old_contents.set_at < contents.set_at);
        *old_contents = contents.clone();
        BinaryEdit {
            file_id: self.inner.global_id(),
            new_contents,
            timestamp: contents.set_at,
        }
    }
}

impl BinaryContents {
    #[inline]
    pub(crate) fn integrate_edit(
        &mut self,
        edit: BinaryEdit,
        ctx: &mut BinaryCtx,
    ) -> bool {
        if self.set_at < edit.timestamp {
            let remote_ts = edit.timestamp;
            self.inner = edit.new_contents;
            self.set_at = remote_ts;
            ctx.edit_clock.max_assign(remote_ts.edited_at);
            true
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn new_local(
        local_id: PeerId,
        inner: Bytes,
        ctx: &mut BinaryCtx,
    ) -> Self {
        Self {
            inner,
            set_at: BinaryEditTimestamp {
                edited_by: local_id,
                edited_at: ctx.edit_clock.tick(),
            },
        }
    }

    #[inline]
    pub(crate) fn new_remote(inner: Bytes, created_by: PeerId) -> Self {
        Self {
            inner,
            set_at: BinaryEditTimestamp {
                edited_at: LamportClock::new(0).tick(),
                edited_by: created_by,
            },
        }
    }
}

impl BinaryEditBacklog {
    #[inline]
    pub(crate) fn insert(&mut self, edit: BinaryEdit) {
        match self.edits.entry(edit.file_id) {
            Entry::Occupied(mut entry) => {
                if entry.get_mut().timestamp < edit.timestamp {
                    *entry.get_mut() = edit;
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(edit);
            },
        }
    }

    #[inline]
    pub(crate) fn take(
        &mut self,
        file_id: GlobalFileId,
    ) -> Option<BinaryEdit> {
        self.edits.remove(&file_id)
    }
}

impl<'a> BinaryStateMut<'a> {
    #[inline]
    pub(crate) fn integrate_edit(&mut self, edit: BinaryEdit) -> bool {
        match self {
            Self::Visible(file) => file.integrate_edit(edit),
            Self::Backlogged(file) => file.integrate_edit(edit),
            Self::Deleted(file) => file.integrate_edit(edit),
        }
    }

    #[inline]
    pub(crate) fn new(
        file_state: PuffFileStateMut<'a>,
        state: StateMut<'a>,
    ) -> Option<Self> {
        match file_state {
            PuffFileStateMut::Visible(file) => {
                match FileMut::new(file, state) {
                    FileMut::Binary(file) => Some(Self::Visible(file)),
                    _ => None,
                }
            },
            PuffFileStateMut::Backlogged(file) => {
                match FileMut::new(file, state) {
                    FileMut::Binary(file) => Some(Self::Backlogged(file)),
                    _ => None,
                }
            },
            PuffFileStateMut::Deleted(file) => {
                match FileMut::new(file, state) {
                    FileMut::Binary(file) => Some(Self::Deleted(file)),
                    _ => None,
                }
            },
        }
    }
}

impl<'a, S> Copy for BinaryFile<'a, S> {}

impl<'a, S> Clone for BinaryFile<'a, S> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
