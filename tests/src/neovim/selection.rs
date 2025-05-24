use ed::Context;
use futures_util::StreamExt;
use neovim::Neovim;
use neovim::tests::ContextExt;

use crate::ed::selection::SelectionEvent;

#[neovim::test]
async fn charwise_simple(ctx: &mut Context<Neovim>) {
    ctx.feedkeys("ihello<Esc>b");

    let mut events = SelectionEvent::new_stream(ctx);

    ctx.feedkeys("v");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Created(0..1));

    ctx.feedkeys("<Right>");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Moved(0..2));

    ctx.feedkeys("<Esc>");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Removed);
}

#[neovim::test]
async fn charwise_past_eof(ctx: &mut Context<Neovim>) {
    ctx.feedkeys("iHello<Esc>0");

    let mut events = SelectionEvent::new_stream(ctx);

    ctx.feedkeys("v");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Created(0..1));

    ctx.feedkeys("w");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Moved(0..5));

    // We're already at EOF, so trying to select one more character shouldn't
    // do anything.
    ctx.feedkeys("<Right>");

    ctx.feedkeys("<Esc>");

    assert_eq!(events.next().await.unwrap(), SelectionEvent::Removed);
}

#[neovim::test]
async fn charwise_past_eol(ctx: &mut Context<Neovim>) {
    ctx.feedkeys("iHello<CR>World<Esc>0<Up>");

    let mut events = SelectionEvent::new_stream(ctx);

    ctx.feedkeys("v");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Created(0..1));

    ctx.feedkeys("e");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Moved(0..5));

    // In Neovim, trying to select past the end of the line will include the
    // following newline in the selection (if there is one).
    ctx.feedkeys("<Right>");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Moved(0..6));

    ctx.feedkeys("<Down>");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Moved(0..11));

    ctx.feedkeys("<Esc>");
    assert_eq!(events.next().await.unwrap(), SelectionEvent::Removed);
}
