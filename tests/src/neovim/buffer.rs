use ed::Context;
use neovim::Neovim;

mod ed_buffer {
    //! Contains the editor-agnostic buffer tests.

    use super::*;
    use crate::ed::buffer;

    #[neovim::test]
    async fn fuzz_edits_10e1(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits(10, ctx).await;
    }

    #[neovim::test]
    async fn fuzz_edits_10e2(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits(100, ctx).await;
    }

    #[neovim::test]
    async fn fuzz_edits_10e3(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits(1_000, ctx).await;
    }

    #[neovim::test]
    async fn fuzz_edits_10e4(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits(10_000, ctx).await;
    }
}
