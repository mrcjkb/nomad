use core::ops::{Deref, DerefMut};

use crate::context::{EventHandle, State};
use crate::{AccessMut, AgentId, Editor, Selection as _, Shared};

/// A wrapper around an [`Editor`]'s [`Selection`](Editor::Selection).
pub struct Selection<'ed, Ed: Editor> {
    inner: Ed::Selection<'ed>,
    state: Shared<State<Ed>>,
}

impl<'ed, Ed: Editor> Selection<'ed, Ed> {
    /// TODO: docs.
    #[inline]
    pub fn on_moved(
        &mut self,
        fun: impl FnMut(&Ed::Selection<'_>, AgentId) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_moved(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// TODO: docs.
    #[inline]
    pub fn on_removed(
        &mut self,
        fun: impl FnMut(Ed::SelectionId, AgentId) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_removed(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// Creates a new `Selection`.
    #[inline]
    pub(crate) fn new(
        inner: Ed::Selection<'ed>,
        state: Shared<State<Ed>>,
    ) -> Self {
        Self { inner, state }
    }

    #[inline]
    fn editor(&self) -> impl AccessMut<Ed> + Clone + 'static {
        self.state.clone().map_mut(Deref::deref, DerefMut::deref_mut)
    }
}

impl<'ed, Ed: Editor> Deref for Selection<'ed, Ed> {
    type Target = Ed::Selection<'ed>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'ed, Ed: Editor> DerefMut for Selection<'ed, Ed> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
