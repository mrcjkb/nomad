use std::borrow::Cow;

use crop::Rope;
use ed_core::ByteOffset;
use ed_core::backend::{self, AgentId, Edit};

use crate::mock::{self, CallbackKind, Callbacks};

/// TODO: docs.
pub struct Buffer<'a> {
    pub(crate) inner: &'a mut BufferInner,
    pub(crate) callbacks: Callbacks,
}

/// TODO: docs.
pub(crate) struct BufferInner {
    pub(crate) contents: Rope,
    pub(crate) id: BufferId,
    pub(crate) name: String,
}

/// TODO: docs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferId(pub(crate) u64);

impl BufferId {
    pub(crate) fn post_inc(&mut self) -> Self {
        let id = *self;
        self.0 += 1;
        id
    }
}

impl backend::Buffer for Buffer<'_> {
    type Backend = mock::Mock;
    type Id = BufferId;
    type EventHandle = mock::EventHandle;

    fn byte_len(&self) -> ByteOffset {
        self.inner.contents.byte_len().into()
    }

    fn id(&self) -> Self::Id {
        self.inner.id
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.inner.name)
    }

    fn on_edited<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Buffer<'_>, &Edit) + 'static,
    {
        self.callbacks.insert(CallbackKind::OnBufferEdited(Box::new(fun)))
    }

    fn on_removed<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Buffer<'_>, AgentId) + 'static,
    {
        self.callbacks.insert(CallbackKind::OnBufferRemoved(Box::new(fun)))
    }

    fn on_saved<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Buffer<'_>, AgentId) + 'static,
    {
        self.callbacks.insert(CallbackKind::OnBufferSaved(Box::new(fun)))
    }
}
