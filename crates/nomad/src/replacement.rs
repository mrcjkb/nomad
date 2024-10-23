use core::ops::Range;

use crate::{ByteOffset, Text};

/// TODO: docs.
pub struct Replacement {
    deleted_range: Range<ByteOffset>,
    inserted_text: Text,
}

impl Replacement {
    /// Returns the range of bytes that were deleted.
    pub fn deleted_range(&self) -> Range<ByteOffset> {
        self.deleted_range.clone()
    }

    /// Returns the text that was inserted.
    pub fn inserted_text(&self) -> &Text {
        &self.inserted_text
    }

    pub(crate) fn new(
        deleted_range: Range<ByteOffset>,
        inserted_text: Text,
    ) -> Self {
        Self { deleted_range, inserted_text }
    }
}
