use ed::Context;
use neovim::Neovim;

use crate::ed::cursor;

#[neovim::test]
fn on_cursor_created_1(ctx: &mut Context<Neovim>) {
    ctx.with_borrowed(cursor::on_cursor_created_1);
}

#[neovim::test]
fn on_cursor_created_2(ctx: &mut Context<Neovim>) {
    ctx.with_borrowed(cursor::on_cursor_created_2);
}

#[neovim::test]
#[ignore = "callbacks registered on CursorMoved{I} are called on the next \
            tick of the event loop, and we don't yet have a way to express \
            this in ed"]
fn on_cursor_moved_1(ctx: &mut Context<Neovim>) {
    ctx.with_borrowed(cursor::on_cursor_moved_1);
}
