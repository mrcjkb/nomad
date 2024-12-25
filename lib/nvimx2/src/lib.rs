//! TODO: docs.

#[cfg(feature = "__neovim")]
pub use backend_neovim as neovim;
#[cfg(feature = "__neovim")]
pub use nvimx2_macros::plugin;
#[doc(inline)]
pub use nvimx_core::*;
