use collab_server::SessionId;
use nomad2::neovim::{self, Neovim};

use crate::Collab;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct JoinSession;

impl JoinSession {
    pub(crate) const NAME: &str = "join";
}

impl neovim::Function for JoinSession {
    const NAME: &str = Self::NAME;
    type Args = SessionId;
    type Module = Collab;
}
