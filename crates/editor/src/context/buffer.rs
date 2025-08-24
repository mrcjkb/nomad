use core::ops::{Deref, DerefMut};

use crate::context::{self, EventHandle, State};
use crate::{AccessMut, AgentId, Buffer as _, Edit, Editor, Shared};

/// A wrapper around an [`Editor`]'s [`Buffer`](Editor::Buffer).
pub struct Buffer<'ed, Ed: Editor> {
    inner: Ed::Buffer<'ed>,
    state: Shared<State<Ed>>,
}

impl<'ed, Ed: Editor> Buffer<'ed, Ed> {
    /// TODO: docs.
    #[inline]
    pub fn for_each_cursor(
        &mut self,
        mut fun: impl FnMut(context::Cursor<'_, Ed>),
    ) {
        let state = self.state.clone();
        self.inner.for_each_cursor(|inner| {
            fun(context::Cursor::new(inner, state.clone()))
        });
    }

    /// TODO: docs.
    #[inline]
    pub fn on_edited(
        &mut self,
        fun: impl FnMut(&Ed::Buffer<'_>, &Edit) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_edited(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// TODO: docs.
    #[inline]
    pub fn on_removed(
        &mut self,
        fun: impl FnMut(Ed::BufferId, AgentId) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_removed(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// TODO: docs.
    #[inline]
    pub fn on_saved(
        &mut self,
        fun: impl FnMut(&Ed::Buffer<'_>, AgentId) + 'static,
    ) -> EventHandle<Ed> {
        let inner = self.inner.on_saved(fun, self.editor());
        EventHandle::new(inner, self.state.clone())
    }

    /// Creates a new `Buffer`.
    #[inline]
    pub(crate) fn new(
        inner: Ed::Buffer<'ed>,
        state: Shared<State<Ed>>,
    ) -> Self {
        Self { inner, state }
    }

    #[inline]
    fn editor(&self) -> impl AccessMut<Ed> + Clone + 'static {
        self.state.clone().map_mut(Deref::deref, DerefMut::deref_mut)
    }
}

impl<'ed, Ed: Editor> Deref for Buffer<'ed, Ed> {
    type Target = Ed::Buffer<'ed>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'ed, Ed: Editor> DerefMut for Buffer<'ed, Ed> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
