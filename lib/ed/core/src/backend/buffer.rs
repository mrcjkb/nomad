use core::ops::Range;
use std::borrow::Cow;

use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::ByteOffset;

/// TODO: docs.
pub trait Buffer {
    /// TODO: docs.
    type EventHandle;

    /// TODO: docs.
    type Id: Clone;

    /// TODO: docs.
    fn byte_len(&self) -> ByteOffset;

    /// TODO: docs.
    fn id(&self) -> Self::Id;

    /// TODO: docs.
    fn name(&self) -> Cow<'_, str>;

    /// TODO: docs.
    fn on_edited<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Self, &Edit) + 'static;

    /// TODO: docs.
    fn on_removed<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Self) + 'static;
}

/// TODO: docs.
pub struct Edit {
    /// TODO: docs.
    pub made_by: (),

    /// TODO: docs.
    pub replacements: SmallVec<[Replacement; 1]>,
}

/// TODO: docs.
pub struct Replacement {
    removed_range: Range<ByteOffset>,
    inserted_text: SmolStr,
}

impl Replacement {
    /// TODO: docs.
    #[inline]
    pub fn inserted_text(&self) -> &str {
        &self.inserted_text
    }

    /// TODO: docs.
    #[inline]
    pub fn new(
        removed_range: Range<ByteOffset>,
        inserted_text: impl Into<SmolStr>,
    ) -> Self {
        Self { removed_range, inserted_text: inserted_text.into() }
    }

    /// TODO: docs.
    #[inline]
    pub fn removed_range(&self) -> Range<ByteOffset> {
        self.removed_range.clone()
    }
}
