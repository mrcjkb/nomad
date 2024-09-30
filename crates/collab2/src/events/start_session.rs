use nomad2::neovim::{self, Neovim};

use crate::Collab;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StartSession;

impl StartSession {
    pub(crate) const NAME: &str = "start";
}

impl neovim::Function for StartSession {
    const NAME: &str = Self::NAME;
    type Args = ();
    type Module = Collab;
}
