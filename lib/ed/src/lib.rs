//! TODO: docs.

#[cfg(feature = "mock")]
pub use backend_mock as mock;
#[cfg(feature = "neovim")]
pub use backend_neovim as neovim;
#[doc(inline)]
pub use ed_core::*;
