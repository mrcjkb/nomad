//! TODO: docs.

mod background_executor;
mod local_executor;
mod neovim;
mod version;

pub use background_executor::NeovimBackgroundExecutor;
pub use local_executor::NeovimLocalExecutor;
pub use neovim::Neovim;
#[doc(hidden)]
pub use nvim_oxi as oxi;
pub use version::NeovimVersion;
#[cfg(feature = "neovim-nightly")]
pub use version::Nightly;
#[cfg(feature = "neovim-0-10")]
pub use version::ZeroDotTen;
