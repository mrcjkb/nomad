/// TODO: docs.
pub trait NeovimVersion: 'static {}

/// TODO: docs.
#[cfg(feature = "neovim-0-10")]
pub struct ZeroDotTen;

/// TODO: docs.
#[cfg(feature = "neovim-nightly")]
pub struct Nightly;

#[cfg(feature = "neovim-0-10")]
impl NeovimVersion for ZeroDotTen {}

#[cfg(feature = "neovim-nightly")]
impl NeovimVersion for Nightly {}
