use ed::EditorCtx;
use ed::backend::Backend;

pub(crate) fn on_cursor_created<Ed: Backend>(_ctx: &mut EditorCtx<'_, Ed>) {}
