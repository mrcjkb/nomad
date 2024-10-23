use crate::autocmd::ShouldDetach;
use crate::ctx::NeovimCtx;
use crate::diagnostics::{DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::neovim::BufferId;
use crate::{Action, ActorId, Module, Replacement};

/// TODO: docs.
pub struct BufAttachArgs {
    /// TODO: docs.
    pub actor_id: ActorId,

    /// TODO: docs.
    pub buffer_id: BufferId,

    /// TODO: docs.
    pub replacement: Replacement,
}

#[derive(Default)]
pub(crate) struct BufAttachMap {}

impl BufAttachMap {
    pub(crate) fn attach<A>(
        &mut self,
        buffer_id: BufferId,
        mut action: A,
        ctx: NeovimCtx<'static>,
    ) where
        A: Action,
        A::Args: From<BufAttachArgs>,
        A::Return: Into<ShouldDetach>,
    {
        let callback = move |buf_attach_args: BufAttachArgs| {
            let args = buf_attach_args.into();
            match action.execute(args).into_result() {
                Ok(res) => res.into(),
                Err(err) => {
                    let mut source = DiagnosticSource::new();
                    source
                        .push_segment(<A::Module as Module>::NAME.as_str())
                        .push_segment(A::NAME.as_str());
                    err.into().emit(Level::Error, source);
                    ShouldDetach::Yes
                },
            }
        };
        todo!();
    }
}
