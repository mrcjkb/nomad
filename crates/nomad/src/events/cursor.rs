use nvim_oxi::api;

use crate::autocmds::{BufEnter, BufEnterArgs};
use crate::buffer_id::BufferId;
use crate::ctx::BufferCtx;
use crate::maybe_result::MaybeResult;
use crate::point::Point;
use crate::{
    Action,
    ActorId,
    ByteOffset,
    Event,
    FnAction,
    Shared,
    ShouldDetach,
};

/// TODO: docs.
#[derive(Clone)]
pub struct Cursor {
    action: CursorAction,
    moved_by: ActorId,
}

/// TODO: docs.
#[derive(Clone, Copy)]
pub enum CursorAction {
    /// The cursor has been moved into the buffer at the given point.
    Created(Point),

    /// The cursor has been moved to the given point.
    Moved(Point),

    /// The cursor has been moved away from the buffer.
    Removed,
}

/// TODO: docs.
pub struct CursorEvent<A> {
    action: A,
}

impl Cursor {
    /// TODO: docs.
    pub fn action(&self) -> CursorAction {
        self.action
    }

    /// TODO: docs.
    pub fn moved_by(&self) -> ActorId {
        self.moved_by
    }
}

impl<A> CursorEvent<A> {
    /// Creates a new [`CursorEvent`] with the given action.
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A> Event for CursorEvent<A>
where
    A: Action + Clone,
    A::Args: From<Cursor>,
    A::Return: Into<ShouldDetach>,
{
    type Ctx<'a> = BufferCtx<'a>;

    fn register(self, ctx: Self::Ctx<'_>) {
        // BufEnter
        // BufLeave
        // CursorMoved
        // CursorMovedI

        let should_detach = Shared::new(ShouldDetach::No);
        let just_entered_buf = Shared::new(false);

        BufEnter::new(FnAction::<_, _, A::Module>::new(
            move |_: BufEnterArgs| {
                just_entered_buf.set(true);
                should_detach.get()
            },
        ))
        .buffer_id(ctx.buffer_id())
        .register((&*ctx).reborrow());
    }
}
