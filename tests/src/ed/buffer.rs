use core::{iter, mem};

use ed::backend::{Buffer, Edit, Replacement};
use ed::{Backend, Context};
use futures_util::stream::{FusedStream, StreamExt};
use rand::Rng;

use crate::utils::{Convert, fuzz};

pub(crate) async fn fuzz_edits_10(ctx: &mut Context<impl Backend>) {
    fuzz_edits(10, ctx).await;
}

pub(crate) async fn fuzz_edits_100(ctx: &mut Context<impl Backend>) {
    fuzz_edits(100, ctx).await;
}

pub(crate) async fn fuzz_edits_1_000(ctx: &mut Context<impl Backend>) {
    fuzz_edits(1_000, ctx).await;
}

pub(crate) async fn fuzz_edits_10_000(ctx: &mut Context<impl Backend>) {
    fuzz_edits(10_000, ctx).await;
}

async fn fuzz_edits(num_epochs: u32, ctx: &mut Context<impl Backend>) {
    let agent_id = ctx.new_agent_id();

    let buf_id = ctx
        .create_and_focus(abs_path::path!("/fuzz.txt"), agent_id)
        .await
        .unwrap();

    let mut edits = Edit::new_stream(buf_id.clone(), ctx);

    // A string to which we'll apply the same edits we apply to the buffer.
    let mut expected_contents = String::new();

    fuzz::run_async(async |rng| {
        for epoch_idx in 0..num_epochs {
            let replacement = gen_replacement(&expected_contents, rng);

            // Apply the replacement to the string.
            expected_contents.replace_range(
                replacement.removed_range().convert(),
                replacement.inserted_text(),
            );

            // Apply the replacement to the buffer.
            ctx.with_borrowed(|ctx| {
                let mut buf = ctx.buffer(buf_id.clone()).unwrap();
                buf.edit(iter::once(replacement.clone()), agent_id);
            });

            // Wait to be notified about the edit we just made.
            let edit = edits.next().await.unwrap();

            // Make sure the edit we got notified about matches the one we
            // applied.
            assert_eq!(edit.made_by, agent_id);
            assert_eq!(&*edit.replacements, &[replacement]);

            // Make sure the buffer's contents are the same as the string.
            ctx.with_borrowed(|ctx| {
                let buf = ctx.buffer(buf_id.clone()).unwrap();
                let buf_contents = buf.get_text(0usize.into()..buf.byte_len());

                if buf_contents != &*expected_contents {
                    panic!(
                        "buffer and string diverged after {} \
                         epochs:\n{buf_contents}\nvs\n{expected_contents}",
                        epoch_idx + 1
                    );
                }
            });
        }
    })
    .await;
}

/// Generates a random replacement to be applied to the given string.
///
/// All [`ByteOffset`](ed::ByteOffset)s in the generated [`Replacement`] are
/// guaranteed to be valid char boundaries in the string.
fn gen_replacement(_s: &str, _rng: &mut impl Rng) -> Replacement {
    todo!();
}

trait EditExt {
    /// Returns a never-endign stream of [`Edit`]s on the buffer with the given
    /// ID.
    fn new_stream<Ed: Backend>(
        buf_id: Ed::BufferId,
        ctx: &mut Context<Ed>,
    ) -> impl FusedStream<Item = Edit> + Unpin + use<Self, Ed> {
        use ed::backend::Buffer;

        let (tx, rx) = flume::unbounded();

        ctx.with_borrowed(|ctx| {
            mem::forget(ctx.buffer(buf_id).unwrap().on_edited(
                move |_buf, edit| {
                    let _ = tx.send(edit.clone());
                },
            ));
        });

        rx.into_stream()
    }
}

impl EditExt for Edit {}
