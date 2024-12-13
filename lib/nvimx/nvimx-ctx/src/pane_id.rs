use core::fmt;
use core::hash::{Hash, Hasher};

use nvim_oxi::api::{self, Window as NvimWindow};

type WinHandle = i32;
use crate::BufferId;

/// TODO: docs.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaneId {
    handle: WinHandle,
}

impl PaneId {
    /// Returns the [`PaneId`] of the currently focused pane.
    pub fn current() -> Self {
        Self::new(NvimWindow::current())
    }

    /// Creates a new [`PaneId`] from the given [`NvimWindow`].
    pub fn new(nvim_window: NvimWindow) -> Self {
        Self { handle: nvim_window.handle() }
    }

    /// Returns an iterator of the [`PaneId`]s of all the currently opened
    /// buffers.
    pub fn opened() -> impl ExactSizeIterator<Item = Self> {
        api::list_wins().map(Self::new)
    }

    pub(crate) fn as_nvim(&self) -> NvimWindow {
        self.handle.into()
    }
}

impl fmt::Debug for PaneId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("PaneId").field(&self.handle).finish()
    }
}

impl Hash for PaneId {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.handle);
    }
}

impl nohash::IsEnabled for PaneId {}
