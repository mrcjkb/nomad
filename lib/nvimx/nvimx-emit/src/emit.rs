use core::iter;

use nvim_oxi::api;
use nvimx_plugin::{Action, Module, Plugin};

use crate::{EmitMessage, Severity};

/// TODO: docs.
pub trait Emit: Sized {
    /// Whether to add the emitted message to the message history.
    const ADD_TO_MESSAGE_HISTORY: bool;

    /// The action this message is about.
    type Action: Action;

    /// TODO: docs.
    fn message(&self) -> EmitMessage;

    /// TODO: docs.
    fn severity(&self) -> Severity;

    /// TODO: docs.
    fn emit(self) {
        let tag_chunks = [
            "[",
            <<Self::Action as Action>::Module as Module>::Plugin::DIAGNOSTIC_NAME,
            ".",
            <Self::Action as Action>::Module::NAME.as_str(),
            ".",
            Self::Action::NAME.as_str(),
            "]"
        ].into_iter().map(|s| (s.into(), Some(self.severity().hl_group())));

        let space_chunk = (" ".into(), None);

        let chunks = tag_chunks
            .chain(iter::once(space_chunk))
            .chain(self.message().chunks);

        // Could fail if the highlight group is invalid.
        let _ = api::echo(
            chunks,
            Self::ADD_TO_MESSAGE_HISTORY,
            &api::opts::EchoOpts::default(),
        );
    }
}
