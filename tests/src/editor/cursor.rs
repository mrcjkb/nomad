use core::mem;

use editor::context::Cursor;
use editor::{AgentId, Buffer, ByteOffset, Context, Cursor as _, Editor};
use futures_util::FutureExt;
use futures_util::stream::{FusedStream, StreamExt};

use crate::editor::{ContextExt, TestEditor};

pub(crate) async fn on_cursor_moved_1(ctx: &mut Context<impl TestEditor>) {
    let agent_id = ctx.new_agent_id();

    let buf_id = ctx.create_and_focus_scratch_buffer(agent_id).await;

    ctx.with_borrowed(|ctx| {
        let mut buf = ctx.buffer(buf_id.clone()).unwrap();
        buf.schedule_insertion(0, "Hello world", agent_id).boxed_local()
    })
    .await;

    let mut events = CursorEvent::new_stream(ctx);

    ctx.with_borrowed(|ctx| {
        let mut buf = ctx.buffer(buf_id.clone()).unwrap();
        buf.for_each_cursor(|mut cursor| {
            let _ = cursor.schedule_move(5, agent_id);
        });
    });

    let movement = match events.next().await.unwrap() {
        CursorEvent::Moved(movement) => movement,
        other => panic!("expected Moved event, got {other:?}"),
    };

    assert_eq!(movement.byte_offset, 5);
    assert_eq!(movement.buffer_id, buf_id);
    assert_eq!(movement.moved_by, agent_id);
}

#[derive(cauchy::Debug, cauchy::PartialEq)]
pub(crate) enum CursorEvent<Ed: Editor> {
    Created(CursorCreation<Ed>),
    Moved(CursorMovement<Ed>),
    Removed(AgentId),
}

#[derive(cauchy::Debug, cauchy::PartialEq)]
pub(crate) struct CursorCreation<Ed: Editor> {
    pub(crate) buffer_id: Ed::BufferId,
    pub(crate) byte_offset: ByteOffset,
    pub(crate) created_by: AgentId,
}

#[derive(cauchy::Debug, cauchy::PartialEq)]
pub(crate) struct CursorMovement<Ed: Editor> {
    pub(crate) buffer_id: Ed::BufferId,
    pub(crate) byte_offset: ByteOffset,
    pub(crate) moved_by: AgentId,
}

impl<Ed: Editor> CursorEvent<Ed> {
    /// Returns a never-ending [`Stream`] of [`CursorEvent`]s.
    #[track_caller]
    pub(crate) fn new_stream(
        ctx: &mut Context<Ed>,
    ) -> impl FusedStream<Item = Self> + Unpin + use<Ed> {
        let (tx, rx) = flume::unbounded();

        ctx.for_each_buffer(|mut buf| {
            buf.for_each_cursor(|mut cursor| {
                // Subscribe to events on each existing cursor.
                subscribe(&mut cursor, tx.clone());
            });
        });

        mem::forget(ctx.on_cursor_created(move |mut cursor, created_by| {
            let event = Self::Created(CursorCreation {
                buffer_id: cursor.buffer_id(),
                byte_offset: cursor.byte_offset(),
                created_by,
            });
            let _ = tx.send(event);

            // Subscribe to events on the newly created cursor.
            subscribe(&mut cursor, tx.clone());
        }));

        rx.into_stream()
    }
}

fn subscribe<Ed: Editor>(
    cursor: &mut Cursor<'_, Ed>,
    tx: flume::Sender<CursorEvent<Ed>>,
) {
    let tx2 = tx.clone();
    mem::forget(cursor.on_moved(move |cursor, moved_by| {
        let event = CursorEvent::Moved(CursorMovement {
            buffer_id: cursor.buffer_id(),
            byte_offset: cursor.byte_offset(),
            moved_by,
        });
        let _ = tx2.send(event);
    }));

    let tx2 = tx.clone();
    mem::forget(cursor.on_removed(move |_cursor_id, removed_by| {
        let event = CursorEvent::Removed(removed_by);
        let _ = tx2.send(event);
    }));
}
