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

    use core::convert::Infallible;
    use core::fmt;
    use core::panic::UnwindSafe;
    use std::panic;
    use std::sync::Arc;

    use ed::Backend;

    use super::*;
    use crate::oxi;

    pub fn sync_test<Out>(
        test_fn: impl FnOnce(&mut Context<Neovim>) -> Out + UnwindSafe,
        test_name: &str,
    ) -> impl FnOnce() -> Out + UnwindSafe
    where
        Out: oxi::IntoResult<()>,
        Out::Error: fmt::Display,
    {
        || Neovim::new_test(test_name).with_ctx(test_fn)
    }

    pub fn async_test<Out>(
        test_fn: impl AsyncFnOnce(&mut Context<Neovim>) -> Out
        + UnwindSafe
        + 'static,
        test_name: &str,
    ) -> impl FnOnce(oxi::tests::TestTerminator) + UnwindSafe
    where
        Out: oxi::IntoResult<()>,
        Out::Error: fmt::Display,
    {
        move |terminator| {
            let terminator = Arc::new(terminator);

            let prev_hook = panic::take_hook();

            panic::set_hook({
                let terminator = terminator.clone();
                Box::new(move |info| {
                    prev_hook(info);
                    let failure =
                        oxi::tests::TestFailure::<Infallible>::Panic(info);
                    terminator.terminate(Err(failure));
                })
            });

            Neovim::new_test(test_name).with_ctx(move |ctx| {
                ctx.spawn_local(async move |ctx| {
                    let res = test_fn(ctx)
                        .await
                        .into_result()
                        .map_err(oxi::tests::TestFailure::Error);
                    terminator.terminate(res);
                })
                .detach();
            })
        }
    }
}
