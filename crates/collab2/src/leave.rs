use nvimx2::AsyncCtx;
use nvimx2::action::AsyncAction;
use nvimx2::command::ToCompletionFn;
use nvimx2::notify::Name;

use crate::backend::CollabBackend;
use crate::collab::Collab;
use crate::yank::{NoActiveSessionError, SessionSelector};

/// TODO: docs.
#[derive(Clone)]
pub struct Leave {
    session_selector: SessionSelector,
}

impl<B: CollabBackend> AsyncAction<B> for Leave {
    const NAME: Name = "leave";

    type Args = ();

    async fn call(
        &mut self,
        _: Self::Args,
        ctx: &mut AsyncCtx<'_, B>,
    ) -> Result<(), NoActiveSessionError<B>> {
        let Some((_, _session_id)) = self.session_selector.select(ctx).await?
        else {
            return Ok(());
        };

        todo!();
    }
}

impl<B: CollabBackend> ToCompletionFn<B> for Leave {
    fn to_completion_fn(&self) {}
}

impl<B: CollabBackend> From<&Collab<B>> for Leave {
    fn from(collab: &Collab<B>) -> Self {
        Self {
            session_selector: SessionSelector::new(collab.sessions.clone()),
        }
    }
}
