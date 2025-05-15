use ed::EditorCtx;
use neovim::Neovim;

use crate::ed::cursor;

#[test]
fn on_cursor_created() {
    let ctx: &mut EditorCtx<'_, Neovim> = todo!();
    cursor::on_cursor_created(ctx);
}
