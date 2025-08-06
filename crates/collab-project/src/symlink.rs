//! TODO: docs.

use std::sync::Arc;

use collab_types::puff;
use puff::file::LocalFileId;
use puff::node::{IsVisible, Visible};

use crate::abs_path::AbsPathBuf;
use crate::fs::{FileContents, PuffFile, PuffFileMut};
use crate::project::{State, StateMut};

/// TODO: docs.
pub struct SymlinkFile<'a, S = Visible> {
    inner: PuffFile<'a, S>,
    state: State<'a>,
}

/// TODO: docs.
pub struct SymlinkFileMut<'a, S = Visible> {
    inner: PuffFileMut<'a, S>,
    state: StateMut<'a>,
}

/// TODO: docs.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct SymlinkContents {
    target_path: Arc<str>,
}

impl<'a, Ctx> SymlinkFile<'a, Ctx> {
    /// TODO: docs.
    #[inline]
    pub fn id(&self) -> LocalFileId {
        self.inner.local_id()
    }

    /// Returns the path of the file that the symlink points to.
    #[inline]
    pub fn target_path(&self) -> &'a str {
        match self.inner.metadata() {
            FileContents::Symlink(symlink_contents) => {
                &symlink_contents.target_path
            },
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn state(&self) -> State<'a> {
        self.state
    }

    #[inline]
    pub(crate) fn inner(&self) -> PuffFile<'a, Ctx> {
        self.inner
    }

    #[track_caller]
    #[inline]
    pub(crate) fn new(inner: PuffFile<'a, Ctx>, state: State<'a>) -> Self {
        debug_assert!(inner.metadata().is_symlink());
        Self { inner, state }
    }
}

impl<'a, S: IsVisible> SymlinkFile<'a, S> {
    /// TODO: docs.
    #[inline]
    pub fn path(&self) -> AbsPathBuf {
        self.inner.path()
    }
}

impl<'a, S> SymlinkFileMut<'a, S> {
    /// TODO: docs.
    #[inline]
    pub fn as_file(&self) -> SymlinkFile<'_, S> {
        SymlinkFile { inner: self.inner.as_file(), state: self.state.as_ref() }
    }

    #[inline]
    pub(crate) fn inner_mut(&mut self) -> &mut PuffFileMut<'a, S> {
        &mut self.inner
    }

    #[inline]
    pub(crate) fn into_inner(self) -> PuffFileMut<'a, S> {
        self.inner
    }

    #[track_caller]
    #[inline]
    pub(crate) fn new(inner: PuffFileMut<'a, S>, state: StateMut<'a>) -> Self {
        debug_assert!(inner.metadata().is_symlink());
        Self { inner, state }
    }
}

impl SymlinkContents {
    #[inline]
    pub(crate) fn new(target_path: Arc<str>) -> Self {
        Self { target_path }
    }
}

impl<'a, Ctx> Copy for SymlinkFile<'a, Ctx> {}

impl<'a, Ctx> Clone for SymlinkFile<'a, Ctx> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
