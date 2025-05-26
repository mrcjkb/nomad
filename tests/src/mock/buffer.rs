use ed::backend::Backend;
use mock::fs::MockFs;
use mock::{ContextExt, Mock};

mod ed_buffer {
    //! Contains the editor-agnostic buffer tests.

    use super::*;
    use crate::ed::buffer;

    #[test]
    fn fuzz_edits_10() {
        Mock::<MockFs>::default()
            .with_ctx(|ctx| ctx.block_on(buffer::fuzz_edits_10));
    }

    #[test]
    fn fuzz_edits_100() {
        Mock::<MockFs>::default()
            .with_ctx(|ctx| ctx.block_on(buffer::fuzz_edits_100));
    }

    #[test]
    fn fuzz_edits_1_000() {
        Mock::<MockFs>::default()
            .with_ctx(|ctx| ctx.block_on(buffer::fuzz_edits_1_000));
    }

    #[test]
    fn fuzz_edits_10_000() {
        Mock::<MockFs>::default()
            .with_ctx(|ctx| ctx.block_on(buffer::fuzz_edits_10_000));
    }
}
