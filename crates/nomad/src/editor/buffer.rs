use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter;
use core::ops::Range;

use cola::{Anchor, Replica};
use crop::Rope;
use flume::Sender;

use super::{BufferId, EditorId};
use crate::streams::{AppliedDeletion, AppliedEdit, AppliedInsertion, Edits};

/// TODO: docs
pub struct Buffer {
    id: BufferId,
    replica: Replica,
    text: Rope,
}

impl Buffer {
    /// TODO: docs
    #[inline]
    pub fn edits(&self) -> Edits {
        self.edits_inner(None)
    }

    /// TODO: docs
    #[inline]
    pub fn edits_filtered(&self, filter_out: EditorId) -> Edits {
        self.edits_inner(Some(filter_out))
    }

    /// TODO: docs
    #[inline]
    pub fn edits_inner(&self, filter_out: Option<EditorId>) -> Edits {
        todo!();
    }

    /// TODO: docs
    #[inline]
    pub async fn new(id: BufferId) -> Self {
        todo!();
    }

    #[inline]
    fn on_bytes(
        &self,
        sender: Sender<AppliedEdit>,
    ) -> impl Fn(ByteEdit) -> bool + 'static {
        move |edit| {
            let byte_range = edit.byte_range();

            let text = rope();

            text.replace(byte_range.clone(), &edit.replacement);

            let replica = replica();

            if !byte_range.is_empty() {
                let del = replica.deleted(byte_range.clone());
                let del = AppliedDeletion::new(del);
                let _ = sender.send(AppliedEdit::Deletion(del));
            }

            let text_len = edit.replacement.len();

            if text_len > 0 {
                let ins = replica.inserted(byte_range.start, text_len);
                let ins = AppliedInsertion::new(ins, edit.replacement);
                let _ = sender.send(AppliedEdit::Insertion(ins));
            }

            false
        }
    }
}

/// TODO: docs
struct BufferInner {
    /// TODO: docs
    crdt: Replica,

    /// TODO: docs
    nvim: nvim::api::Buffer,

    /// TODO: docs
    text: Rope,
}

impl BufferInner {
    /// TODO: docs
    ///
    /// # Panics
    ///
    /// Panics if either the start or end anchor cannot be resolved to a byte
    /// offset in the buffer.
    #[track_caller]
    #[inline]
    fn apply_local_deletion(
        &mut self,
        delete_range: Range<Anchor>,
    ) -> Option<AppliedDeletion> {
        let Some(start_offset) = self.crdt.resolve_anchor(delete_range.start)
        else {
            panic_couldnt_resolve_anchor(&delete_range.start);
        };

        let Some(end_offset) = self.crdt.resolve_anchor(delete_range.end)
        else {
            panic_couldnt_resolve_anchor(&delete_range.end);
        };

        if start_offset == end_offset {
            return None;
        }

        assert!(start_offset < end_offset);

        let start_point = self.point_of_offset(start_offset);

        let end_point = self.point_of_offset(end_offset);

        self.text.delete(start_offset..end_offset);

        let deletion = self.crdt.deleted(start_offset..end_offset);

        self.nvim
            .set_text(
                start_point.row..end_point.row,
                start_point.col,
                end_point.col,
                iter::empty::<nvim::String>(),
            )
            .expect("start and end points are within bounds");

        Some(AppliedDeletion::new(deletion))
    }

    /// TODO: docs
    ///
    /// # Panics
    ///
    /// Panics if the anchor cannot be resolved to a byte offset in the buffer.
    #[track_caller]
    #[inline]
    fn apply_local_insertion(
        &mut self,
        insert_at: Anchor,
        text: String,
    ) -> AppliedInsertion {
        let Some(byte_offset) = self.crdt.resolve_anchor(insert_at) else {
            panic_couldnt_resolve_anchor(&insert_at);
        };

        let point = self.point_of_offset(byte_offset);

        self.text.insert(byte_offset, &text);

        let insertion = self.crdt.inserted(byte_offset, text.len());

        let Point { row, col } = point;

        self.nvim
            .set_text(row..row, col, col, iter::once(&*text))
            .expect("row and col are within bounds");

        AppliedInsertion::new(insertion, text)
    }

    /// Transforms the 1-dimensional byte offset into a 2-dimensional
    /// [`Point`].
    #[inline]
    fn point_of_offset(&self, byte_offset: ByteOffset) -> Point {
        let row = self.text.line_of_byte(byte_offset);
        let row_offset = self.text.byte_of_line(row);
        let col = byte_offset - row_offset;
        Point { row, col }
    }
}

#[inline(never)]
fn panic_couldnt_resolve_anchor(anchor: &Anchor) -> ! {
    panic!("{anchor:?} couldn't be resolved");
}

/// TODO: docs
struct Point {
    /// TODO: docs
    row: usize,

    /// TODO: docs
    col: ByteOffset,
}

#[derive(Clone)]
struct EditQueue {
    inner: Rc<RefCell<VecDeque<PendingEdit>>>,
}

/// TODO: docs
enum PendingEdit {
    Local(PendingLocalEdit),
    Remote(PendingRemoteEdit),
}

/// TODO: docs
enum PendingLocalEdit {
    Insertion(LocalInsertion),
    Deletion(LocalDeletion),
}

/// TODO: docs
struct LocalInsertion {
    insert_at: Anchor,
    text: String,
}

/// TODO: docs
struct LocalDeletion {
    range: Range<Anchor>,
}

/// TODO: docs
enum PendingRemoteEdit {
    Insertion(RemoteInsertion),
    Deletion(RemoteDeletion),
}

/// TODO: docs
struct RemoteInsertion {
    inner: cola::Insertion,
    text: String,
}

/// TODO: docs
struct RemoteDeletion {
    inner: cola::Deletion,
}

fn rope() -> &'static mut Rope {
    todo!()
}

fn replica() -> &'static mut Replica {
    todo!()
}

type ByteOffset = usize;

/// TODO: docs
struct ByteEdit {
    start: ByteOffset,
    end: ByteOffset,
    replacement: String,
}

impl ByteEdit {
    #[inline]
    fn byte_range(&self) -> Range<usize> {
        self.start..self.end
    }
}
