use ed::backend::Backend;
use mock::Mock;
use mock::fs::MockFs;

use crate::ed::cursor;

#[test]
fn on_cursor_created() {
    Mock::<MockFs>::default().with_ctx(cursor::on_cursor_created);
}
