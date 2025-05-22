//! TODO: docs.

use ed::Context;

use crate::Neovim;

/// TODO: docs.
pub trait ContextExt {
    /// TODO: docs.
    fn feedkeys(&mut self, keys: &str);
}

impl ContextExt for ed::Context<Neovim, ed::NotBorrowed> {
    fn feedkeys(&mut self, keys: &str) {
        self.with_borrowed(|ctx| ctx.feedkeys(keys))
    }
}

#[doc(hidden)]
pub mod test_macro {
    //! The functions in this module are not part of the crate's public API and
    //! should only be called by the `#[neovim::test]` macro.

    use core::fmt;
    use core::panic::UnwindSafe;

    use ed::Backend;

    use super::*;
    use crate::oxi;

    pub fn sync_test<Out>(
        test_fn: impl FnOnce(&mut Context<Neovim>) -> Out + UnwindSafe,
        test_name: &str,
    ) -> impl (FnOnce() -> Out) + UnwindSafe
    where
        Out: oxi::IntoResult<()>,
        Out::Error: fmt::Display,
    {
        || Neovim::new_test(test_name).with_ctx(test_fn)
    }

    pub fn async_test<'ctx, Fut, Out>(
        _test_fn: impl FnOnce(&'ctx mut Context<Neovim>) -> Fut + UnwindSafe,
        _test_name: &str,
    ) -> impl (FnOnce(oxi::tests::TestTerminator)) + UnwindSafe
    where
        Fut: Future<Output = Out> + 'ctx,
        Out: oxi::IntoResult<()>,
        Out::Error: fmt::Display,
    {
        |_terminator| ()
    }
}
