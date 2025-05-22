use core::mem;
use core::ops::Range;

use ed::backend::{Buffer, Selection};
use ed::{Backend, Context};
use futures_lite::Stream;

use crate::utils::Convert;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SelectionEvent {
    Created(Range<usize>),
    Moved(Range<usize>),
    Removed,
}

impl SelectionEvent {
    /// Returns a never-ending [`Stream`] of [`SelectionEvent`]s on the current
    /// buffer.
    #[track_caller]
    pub(crate) fn new_stream(
        ctx: &mut Context<impl Backend>,
    ) -> impl Stream<Item = Self> {
        let (tx, rx) = flume::unbounded();

        let buffer_id = ctx.with_borrowed(|ctx| {
            ctx.current_buffer().expect("no current buffer").id()
        });

        mem::forget(ctx.on_selection_created(
            move |selection, _created_by| {
                if selection.buffer_id() != buffer_id {
                    return;
                }

                tx.send(Self::Created(selection.byte_range().convert()))
                    .unwrap();

                mem::forget(selection.on_moved({
                    let tx = tx.clone();
                    move |selection, _moved_by| {
                        tx.send(Self::Moved(selection.byte_range().convert()))
                            .unwrap();
                    }
                }));

                mem::forget(selection.on_removed({
                    let tx = tx.clone();
                    move |_selection_id, _moved_by| {
                        tx.send(Self::Removed).unwrap();
                    }
                }));
            },
        ));

        rx.into_stream()
    }
}
