//! TODO: docs.

#[cfg(feature = "__neovim")]
pub use backend_neovim as neovim;
#[doc(inline)]
pub use nvimx_core::*;
