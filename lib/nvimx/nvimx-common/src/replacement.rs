use core::ops::Range;

use crate::byte_offset::ByteOffset;
use crate::text::Text;

/// TODO: docs.
#[derive(Debug, Clone)]
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

    /// Creates a new `Replacement`.
    pub fn new(deleted_range: Range<ByteOffset>, inserted_text: Text) -> Self {
        Self { deleted_range, inserted_text }
    }
}

impl From<eerie::Hunk> for Replacement {
    fn from(hunk: eerie::Hunk) -> Self {
        let deleted_start = hunk.removed_range.start.into();
        let deleted_end = hunk.removed_range.start.into();
        let mut inserted_text = Text::new();
        inserted_text.push_str(hunk.inserted_text.as_str());
        Self { deleted_range: deleted_start..deleted_end, inserted_text }
    }
}

impl From<Replacement> for eerie::Hunk {
    fn from(replacement: Replacement) -> Self {
        let removed_start = replacement.deleted_range.start.into_u64();
        let removed_end = replacement.deleted_range.end.into_u64();
        Self {
            removed_range: removed_start..removed_end,
            inserted_text: eerie::Text::new(replacement.inserted_text),
        }
    }
}
