//! TODO: docs.

#[cfg(feature = "mock")]
pub use backend_mock as mock;
#[cfg(feature = "__neovim")]
pub use backend_neovim as neovim;
#[doc(inline)]
pub use nvimx_core::*;
