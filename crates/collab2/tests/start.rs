#![allow(missing_docs)]

use auth::Auth;
use collab2::Collab;
use collab2::backend::test::CollabTestBackend;
use collab2::start::StartError;
use nvimx2::action::AsyncAction;
use nvimx2::backend::Backend;
use nvimx2::tests::{TestBackend, fs};

#[test]
fn cannot_start_session_if_not_logged_in() {
    let fs = fs!();
    let backend = CollabTestBackend::new(TestBackend::new(fs));
    backend.with_async_ctx(async move |ctx| {
        let collab = Collab::from(&Auth::default());
        let err = collab.start().call((), ctx).await.unwrap_err();
        assert!(matches!(err, StartError::UserNotLoggedIn(_)));
    });
}

// block_on() is a method provided by the `Backend` trait w/ a default impl
// that can only be implemented in the `nvimx-core` crate.
//
// That one takes an AsyncFnOnce(AsyncCtx<Self>) closure. There should also be
// one for sync closures, which takes a FnOnce(EditorCtx<Self>).

// The goal is to be able to run mock tests using the regular `#[test]` macro.
// That way we don't have anything to document, onboarding because moot, using
// other testing frameworks becomes obvious, we can use other test macros (e.g.
// quickcheck), etc.
//
// And there would only be 3 scenarios I can think of where we'd even consider
// using a proc macro for (excluding tests that run under Neovim, ofc):
//
// - async tests. Actually, for this one we might have to.. The executor that
// blocks on the test should be the same that's used to spawn new tasks
// (right?). If that can be relaxed, we can use any other executor as long as
// they are deterministic.
//
// - constructing a *Backend. This is possible for Neovim tests because the
// NeovimBackend has an `init()` method that doesn't take any parameters, but
// the `MockBackend` needs the Fs, (and it might also be modified before it's
// placed into a `nvimx::NeovimCtx`), which makes it unfeasible to construct it
// in a macro.
//
// - fuzzing tests. Basically utilities for generating an RNG from a seed.
// Shouldn't be too difficult to do w/ a builder pattern though, e.g:
//
// ```
// let fs = fs! { .. };
//
// let backend: FuzzingBackend<MockBackend> = MockBackend::new(fs)
//     .into_fuzzing()
//     .with_seed(1234567890);
// ```
//
// where:
//
// ```
// pub struct FuzzingBackend<T: Backend> { .. }
//
// impl<T: Backend> BackendAdapter for FuzzingBackend<T> {
//     type Base = T;
//
//     // Other stuff.
// }
// ```
