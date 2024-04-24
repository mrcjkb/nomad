use alloc::rc::Rc;
use core::cell::RefCell;
use core::ops::Range;

use cola::{Anchor, Deletion, Insertion, Replica};
use crop::Rope;

use crate::streams::{AppliedDeletion, AppliedInsertion};
use crate::{BufferSnapshot, ByteOffset, Edit, Replacement};

/// TODO: docs
#[derive(Clone)]
pub(crate) struct BufferState {
    /// TODO: docs
    inner: Rc<RefCell<BufferInner>>,
}

impl BufferState {
    #[inline]
    pub fn edit<E>(&self, edit: E) -> E::Diff
    where
        E: Edit<BufferInner>,
    {
        self.with_mut(|inner| inner.edit(edit))
    }

    #[inline]
    pub fn new(text: impl Into<Rope>, replica: Replica) -> Self {
        Self { inner: Rc::new(RefCell::new(BufferInner::new(text, replica))) }
    }

    #[inline]
    pub fn snapshot(&self) -> BufferSnapshot {
        self.with(|inner| inner.snapshot())
    }

    #[inline]
    pub(crate) fn with<R>(&self, f: impl FnOnce(&BufferInner) -> R) -> R {
        let inner = self.inner.borrow();
        f(&inner)
    }

    #[inline]
    pub(crate) fn with_mut<R>(
        &self,
        f: impl FnOnce(&mut BufferInner) -> R,
    ) -> R {
        let mut inner = self.inner.borrow_mut();
        f(&mut inner)
    }
}

/// TODO: docs
#[derive(Clone)]
pub(super) struct BufferInner {
    /// TODO: docs
    replica: Replica,

    /// TODO: docs
    text: Rope,
}

impl BufferInner {
    /// TODO: docs
    #[inline]
    pub fn delete(&mut self, range: Range<ByteOffset>) -> Deletion {
        let range: Range<usize> = range.start.into()..range.end.into();
        self.text.delete(range.clone());
        self.replica.deleted(range)
    }

    /// TODO: docs
    #[inline]
    pub fn edit<E>(&mut self, edit: E) -> E::Diff
    where
        E: Edit<Self>,
    {
        edit.apply(self)
    }

    /// TODO: docs
    #[inline]
    pub fn insert(&mut self, offset: ByteOffset, text: &str) -> Insertion {
        self.text.insert(offset.into(), text);
        self.replica.inserted(offset.into(), text.len())
    }

    #[inline]
    fn new(text: impl Into<Rope>, replica: Replica) -> Self {
        let text = text.into();

        assert_eq!(
            text.byte_len(),
            replica.len(),
            "text and replica out of sync"
        );

        Self { replica, text }
    }

    /// Returns an exclusive reference to the buffer's [`Replica`].
    #[inline]
    pub(crate) fn replica_mut(&mut self) -> &mut Replica {
        &mut self.replica
    }

    /// TODO: docs
    #[inline]
    pub fn resolve_anchor(&self, anchor: &Anchor) -> Option<ByteOffset> {
        self.replica.resolve_anchor(*anchor).map(ByteOffset::new)
    }

    /// Returns a shared reference to the buffer's [`Rope`].
    #[inline]
    pub(crate) fn rope(&self) -> &Rope {
        &self.text
    }

    /// Returns an exclusive reference to the buffer's [`Rope`].
    #[inline]
    pub(crate) fn rope_mut(&mut self) -> &mut Rope {
        &mut self.text
    }

    /// TODO: docs
    #[inline]
    pub fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot::new(self.replica.clone(), self.text.clone())
    }
}

impl Edit<BufferInner> for &Replacement<ByteOffset> {
    type Diff = (Option<AppliedDeletion>, Option<AppliedInsertion>);

    #[inline]
    fn apply(self, buf: &mut BufferInner) -> Self::Diff {
        let mut applied_del = None;
        let mut applied_ins = None;

        if !self.range().is_empty() {
            let del = buf.delete(self.range());
            applied_del = Some(AppliedDeletion::new(del));
        }

        if !self.text().is_empty() {
            let ins = buf.insert(self.range().start, self.text());
            applied_ins =
                Some(AppliedInsertion::new(ins, self.text().to_owned()));
        }

        (applied_del, applied_ins)
    }
}
