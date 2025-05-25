use ed::Context;
use neovim::Neovim;

use crate::ed::cursor;

#[neovim::test]
async fn on_cursor_created_1(ctx: &mut Context<Neovim>) {
    cursor::on_cursor_created_1(ctx).await;
}

#[neovim::test]
async fn on_cursor_created_2(ctx: &mut Context<Neovim>) {
    cursor::on_cursor_created_2(ctx).await;
}

#[neovim::test]
async fn on_cursor_moved_1(ctx: &mut Context<Neovim>) {
    cursor::on_cursor_moved_1(ctx).await;
}
