use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::ops::Range;

use async_broadcast::{InactiveReceiver, Sender};
use cola::{Anchor, Replica, ReplicaId};
use crop::Rope;
use nvim::api;

use super::BufferState;
use crate::runtime::spawn;
use crate::streams::{AppliedDeletion, AppliedEdit, AppliedInsertion, Edits};
use crate::{
    Apply,
    BufferSnapshot,
    ByteOffset,
    EditorId,
    IntoCtx,
    NvimBuffer,
    Replacement,
};

/// TODO: docs
pub struct Buffer {
    /// TODO: docs
    applied_queue: AppliedEditQueue,

    /// TODO: docs
    nvim: NvimBuffer,

    /// TODO: docs
    receiver: InactiveReceiver<AppliedEdit>,

    /// TODO: docs
    sender: Sender<AppliedEdit>,

    /// TODO: docs
    state: BufferState,
}

impl Buffer {
    /// TODO: docs
    #[inline]
    fn attach(&self) {
        self.nvim.on_edit(self.on_edit());
    }

    /// TODO: docs
    #[inline]
    pub fn create(text: &str, replica: Replica) -> Self {
        let state = BufferState::new(text, replica);

        let mut buf = NvimBuffer::create();

        let Ok(()) = buf.inner_mut().set_lines(.., true, text.lines()) else {
            unreachable!()
        };

        let Ok(()) = api::Window::current().set_buf(buf.inner()) else {
            unreachable!()
        };

        Self::new(state, buf)
    }

    /// TODO: docs
    #[inline]
    pub fn edit<E>(
        &mut self,
        edit: E,
        _editor_id: EditorId,
    ) -> <Self as Apply<E>>::Diff
    where
        Self: Apply<E>,
    {
        self.apply(edit)
    }

    /// TODO: docs
    #[inline]
    pub fn edits(&self) -> Edits {
        Edits::new(self.receiver.activate_cloned())
    }

    /// TODO: docs
    ///
    /// # Panics
    ///
    /// todo.
    #[inline]
    pub fn from_id(replica_id: ReplicaId, buffer: NvimBuffer) -> Self {
        let text = Rope::try_from(&buffer).expect("");
        let replica = Replica::new(replica_id, text.byte_len());
        Self::new(BufferState::new(text, replica), buffer)
    }

    #[inline]
    fn new(state: BufferState, bound_to: NvimBuffer) -> Self {
        let (sender, receiver) = async_broadcast::broadcast(32);

        let this = Self {
            applied_queue: AppliedEditQueue::new(),
            nvim: bound_to,
            state,
            receiver: receiver.deactivate(),
            sender,
        };

        this.attach();

        this
    }

    #[inline]
    fn on_edit(&self) -> impl Fn(&Replacement<ByteOffset>) + 'static {
        let applied_queue = self.applied_queue.clone();
        let buffer = self.state.clone();
        let sender = self.sender.clone();

        move |replacement| {
            // If the change was caused by an edit we already applied we
            // mustn't apply it again.
            if let Some(edit) = applied_queue.pop_front() {
                broadcast_edit(&sender, edit);
            }
            // The change was either caused by the user or by another plugin,
            // so we apply it to our buffer to keep it in sync.
            else {
                let (del, ins) = buffer.edit(replacement);

                let id = EditorId::unknown();

                if let Some(deletion) = del {
                    let edit = AppliedEdit::deletion(deletion, id);
                    broadcast_edit(&sender, edit);
                }

                if let Some(insertion) = ins {
                    let edit = AppliedEdit::insertion(insertion, id);
                    broadcast_edit(&sender, edit);
                }
            }
        }
    }

    /// TODO: docs
    #[inline]
    pub fn snapshot(&self) -> BufferSnapshot {
        self.state.snapshot()
    }
}

/// TODO: docs
#[inline]
fn broadcast_edit(sender: &Sender<AppliedEdit>, edit: AppliedEdit) {
    if sender.receiver_count() > 0 {
        let sender = sender.clone();

        spawn(async move {
            if sender.receiver_count() > 0 {
                let _ = sender.broadcast_direct(edit).await;
            }
        });
    }
}

#[derive(Clone)]
struct AppliedEditQueue {
    inner: Rc<RefCell<VecDeque<AppliedEdit>>>,
}

impl AppliedEditQueue {
    #[inline]
    fn new() -> Self {
        Self { inner: Rc::new(RefCell::new(VecDeque::new())) }
    }

    #[inline]
    fn pop_front(&self) -> Option<AppliedEdit> {
        self.inner.borrow_mut().pop_front()
    }

    #[inline]
    fn push_back(&self, edit: AppliedEdit) {
        self.inner.borrow_mut().push_back(edit);
    }
}

impl Apply<Replacement<ByteOffset>> for Buffer {
    type Diff = ();

    #[inline]
    fn apply(&mut self, repl: Replacement<ByteOffset>) -> Self::Diff {
        let point_range =
            self.state.with(|inner| repl.range().into_ctx(inner.rope()));

        self.state.with_mut(|inner| {
            let range = repl.range().start.into()..repl.range().end.into();
            inner.rope_mut().replace(range.clone(), repl.text());
            inner.replica_mut().deleted(range.clone());
            inner.replica_mut().inserted(range.start, repl.text().len());
        });

        self.nvim.edit(repl.map_range(|_| point_range));
    }
}

impl Apply<Replacement<Anchor>> for Buffer {
    type Diff = ();

    #[inline]
    fn apply(&mut self, repl: Replacement<Anchor>) -> Self::Diff {
        let anchor_range = repl.range();

        let (start, end) = self.state.with(|inner| {
            let start = inner.resolve_anchor(&anchor_range.start);
            let end = inner.resolve_anchor(&anchor_range.end);
            (start, end)
        });

        if let (Some(start), Some(end)) = (start, end) {
            self.apply(repl.map_range(|_| start..end));
        }
    }
}

impl<T: AsRef<str>> Apply<(&cola::Insertion, T)> for Buffer {
    type Diff = ();

    #[inline]
    fn apply(
        &mut self,
        (insertion, text): (&cola::Insertion, T),
    ) -> Self::Diff {
        let maybe_point = self.state.with_mut(|inner| {
            let offset = inner.replica_mut().integrate_insertion(insertion)?;
            inner.rope_mut().insert(offset, text.as_ref());
            Some(ByteOffset::new(offset).into_ctx(inner.rope()))
        });

        if let Some(point) = maybe_point {
            self.nvim.edit(Replacement::insertion(point, text.as_ref()));
        }
    }
}

impl Apply<&cola::Deletion> for Buffer {
    type Diff = ();

    #[inline]
    fn apply(&mut self, deletion: &cola::Deletion) -> Self::Diff {
        let byte_ranges = self.state.with_mut(|inner| {
            inner.replica_mut().integrate_deletion(deletion)
        });

        let point_ranges = byte_ranges
            .iter()
            .cloned()
            .map(|range| {
                ByteOffset::from(range.start)..ByteOffset::from(range.end)
            })
            .map(|range| {
                self.state.with(|inner| range.into_ctx(inner.rope()))
            });

        for point_range in point_ranges.rev() {
            self.nvim.edit(Replacement::deletion(point_range));
        }

        for byte_range in byte_ranges.into_iter().rev() {
            self.state.with_mut(|inner| inner.rope_mut().delete(byte_range));
        }
    }
}
