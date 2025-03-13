use std::borrow::Cow;

use crop::Rope;
use ed_core::{ByteOffset, backend};

/// TODO: docs.
pub struct Buffer {
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

impl backend::Buffer for Buffer {
    type Id = BufferId;

    fn byte_len(&self) -> ByteOffset {
        self.contents.byte_len().into()
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}
