//! TODO: docs.

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
