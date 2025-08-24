use crate::context::{BorrowState, Context, State};
use crate::{Editor, Shared};

/// A wrapper around an [`Editor`]'s [`EventHandle`](Editor::EventHandle) that
/// executes the editor's [`remove_event`](Editor::remove_event) method when
/// dropped.
pub struct EventHandle<Ed: Editor> {
    inner: Option<Ed::EventHandle>,
    state: Shared<State<Ed>>,
}

impl<Ed: Editor> EventHandle<Ed> {
    /// Creates a new [`EventHandle`] from the given inner
    /// [`EventHandle`](Editor::EventHandle) and context.
    #[inline]
    pub fn new(
        inner: Ed::EventHandle,
        ctx: &mut Context<Ed, impl BorrowState>,
    ) -> Self {
        Self { inner: Some(inner), state: ctx.state_handle() }
    }
}

impl<Ed: Editor> Drop for EventHandle<Ed> {
    fn drop(&mut self) {
        self.state.with_mut(|state| {
            state.remove_event(self.inner.take().expect("only taken on Drop"));
        });
    }
}
