use core::mem;
use core::time::Duration;

use abs_path::path;
use ed::backend::{AgentId, Backend, Buffer, Cursor, Replacement};
use ed::{ByteOffset, Context};
use futures_util::stream::{FusedStream, StreamExt};
use futures_util::{FutureExt, select_biased};

use crate::ed::{ContextExt, TestEditor};

pub(crate) async fn on_cursor_created_1(ctx: &mut Context<impl TestEditor>) {
    let agent_id = ctx.new_agent_id();

    let mut events = CursorEvent::new_stream(ctx);

    // Focusing the buffer should create a cursor.
    let foo_id = ctx.create_and_focus_scratch_buffer(agent_id).await;

    match events.next().await.unwrap() {
        CursorEvent::Created(creation) => {
            assert_eq!(creation.buffer_id, foo_id);
            assert_eq!(creation.created_by, agent_id)
        },
        other => panic!("expected Created event, got {other:?}"),
    }
}

pub(crate) async fn on_cursor_created_2(ctx: &mut Context<impl TestEditor>) {
    let agent_id = ctx.new_agent_id();

    let foo_id = ctx.create_and_focus_scratch_buffer(agent_id).await;

    let mut events = CursorEvent::new_stream(ctx);

    // foo.txt is currently focused, so focusing it again shouldn't do
    // anything.
    ctx.with_borrowed(|ctx| ctx.buffer(foo_id).unwrap().focus(agent_id));

    // Now create and focus bar.txt, which should create a cursor.
    let bar_id =
        ctx.create_and_focus(path!("/bar.txt"), agent_id).await.unwrap();

    match events.next().await.unwrap() {
        CursorEvent::Created(creation) => {
            assert_eq!(creation.buffer_id, bar_id);
            assert_eq!(creation.created_by, agent_id);
        },
        other => panic!("expected Created event, got {other:?}"),
    }
}

pub(crate) async fn on_cursor_moved_1(ctx: &mut Context<impl TestEditor>) {
    let agent_id = ctx.new_agent_id();

    let mut events = CursorEvent::new_stream(ctx);

    let foo_id = ctx.create_and_focus_scratch_buffer(agent_id).await;

    ctx.with_borrowed(|ctx| {
        let mut foo = ctx.buffer(foo_id.clone()).unwrap();
        foo.edit([Replacement::insertion(0usize, "Hello world")], agent_id);
    });

    // Drain the event stream.
    let sleep = async_io::Timer::after(Duration::from_millis(500));
    select_biased! {
        _event = events.select_next_some() => {},
        _now = FutureExt::fuse(sleep) => {},
    }

    ctx.with_borrowed(|ctx| {
        let mut foo = ctx.buffer(foo_id.clone()).unwrap();
        foo.for_each_cursor(|mut cursor| {
            cursor.r#move(5usize.into(), agent_id);
        });
    });

    match events.next().await.unwrap() {
        CursorEvent::Moved(movement) => {
            assert_eq!(movement.byte_offset, 5usize);
            assert_eq!(movement.buffer_id, foo_id);
            assert_eq!(movement.moved_by, agent_id);
        },
        other => panic!("expected Moved event, got {other:?}"),
    }
}

#[derive(cauchy::Debug, cauchy::PartialEq)]
pub(crate) enum CursorEvent<Ed: Backend> {
    Created(CursorCreation<Ed>),
    Moved(CursorMovement<Ed>),
    Removed(AgentId),
}

#[derive(cauchy::Debug, cauchy::PartialEq)]
pub(crate) struct CursorCreation<Ed: Backend> {
    pub(crate) buffer_id: Ed::BufferId,
    pub(crate) byte_offset: ByteOffset,
    pub(crate) created_by: AgentId,
}

#[derive(cauchy::Debug, cauchy::PartialEq)]
pub(crate) struct CursorMovement<Ed: Backend> {
    pub(crate) buffer_id: Ed::BufferId,
    pub(crate) byte_offset: ByteOffset,
    pub(crate) moved_by: AgentId,
}

impl<Ed: Backend> CursorEvent<Ed> {
    /// Returns a never-ending [`Stream`] of [`CursorEvent`]s on the current
    /// buffer.
    #[track_caller]
    pub(crate) fn new_stream(
        ctx: &mut Context<Ed>,
    ) -> impl FusedStream<Item = Self> + Unpin + use<Ed> {
        let (tx, rx) = flume::unbounded();

        mem::forget(ctx.on_cursor_created(move |cursor, created_by| {
            let event = Self::Created(CursorCreation {
                buffer_id: cursor.buffer_id(),
                byte_offset: cursor.byte_offset(),
                created_by,
            });
            let _ = tx.send(event);

            mem::forget(cursor.on_moved({
                let tx = tx.clone();
                move |cursor, moved_by| {
                    let event = Self::Moved(CursorMovement {
                        buffer_id: cursor.buffer_id(),
                        byte_offset: cursor.byte_offset(),
                        moved_by,
                    });
                    let _ = tx.send(event);
                }
            }));

            mem::forget(cursor.on_removed({
                let tx = tx.clone();
                move |_selection_id, removed_by| {
                    let event = Self::Removed(removed_by);
                    let _ = tx.send(event);
                }
            }));
        }));

        rx.into_stream()
    }
}
