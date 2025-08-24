use core::ops::{Deref, DerefMut};

use crate::context::{EventHandle, State};
use crate::{AccessMut, AgentId, Cursor as _, Editor, Shared};

/// A wrapper around an [`Editor`]'s [`Cursor`](Editor::Cursor).
pub struct Cursor<'ed, Ed: Editor> {
    inner: Ed::Cursor<'ed>,
    state: Shared<State<Ed>>,
}

impl<'ed, Ed: Editor> Cursor<'ed, Ed> {
    /// TODO: docs.
    #[inline]
    pub fn on_moved(
        &mut self,
        fun: impl FnMut(&Ed::Cursor<'_>, AgentId) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_moved(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// TODO: docs.
    #[inline]
    pub fn on_removed(
        &mut self,
        fun: impl FnMut(Ed::CursorId, AgentId) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_removed(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// Creates a new `Cursor`.
    #[inline]
    pub(crate) fn new(
        inner: Ed::Cursor<'ed>,
        state: Shared<State<Ed>>,
    ) -> Self {
        Self { inner, state }
    }

    #[inline]
    fn editor(&self) -> impl AccessMut<Ed> + Clone + 'static {
        self.state.clone().map_mut(Deref::deref, DerefMut::deref_mut)
    }
}

impl<'ed, Ed: Editor> Deref for Cursor<'ed, Ed> {
    type Target = Ed::Cursor<'ed>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'ed, Ed: Editor> DerefMut for Cursor<'ed, Ed> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
