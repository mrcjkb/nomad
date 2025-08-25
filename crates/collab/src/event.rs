use core::ops::Range;

use abs_path::AbsPathBuf;
use editor::{ByteOffset, Editor, Replacement};
use fs::{DirectoryEvent, FileEvent};
use smallvec::SmallVec;

/// TODO: docs.
#[derive(cauchy::Debug)]
pub(crate) enum Event<Ed: Editor> {
    /// TODO: docs.
    Buffer(BufferEvent<Ed>),

    /// TODO: docs.
    Cursor(CursorEvent<Ed>),

    /// TODO: docs.
    Directory(DirectoryEvent<Ed::Fs>),

    /// TODO: docs.
    File(FileEvent<Ed::Fs>),

    /// TODO: docs.
    Selection(SelectionEvent<Ed>),
}

#[derive(cauchy::Debug)]
pub(crate) enum BufferEvent<Ed: Editor> {
    Created(Ed::BufferId, AbsPathBuf),
    Edited(Ed::BufferId, SmallVec<[Replacement; 1]>),
    Removed(Ed::BufferId),
    Saved(Ed::BufferId),
}

#[derive(cauchy::Debug)]
pub(crate) struct CursorEvent<Ed: Editor> {
    pub(crate) cursor_id: Ed::CursorId,
    pub(crate) kind: CursorEventKind<Ed>,
}

#[derive(cauchy::Debug)]
pub(crate) enum CursorEventKind<Ed: Editor> {
    Created(Ed::BufferId, ByteOffset),
    Moved(ByteOffset),
    Removed,
}

#[derive(cauchy::Debug)]
pub(crate) struct SelectionEvent<Ed: Editor> {
    pub(crate) selection_id: Ed::SelectionId,
    pub(crate) kind: SelectionEventKind<Ed>,
}

#[derive(cauchy::Debug)]
pub(crate) enum SelectionEventKind<Ed: Editor> {
    Created(Ed::BufferId, Range<ByteOffset>),
    Moved(Range<ByteOffset>),
    Removed,
}
