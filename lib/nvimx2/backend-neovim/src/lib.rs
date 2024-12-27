//! TODO: docs.

pub mod api;
mod background_executor;
mod convert;
mod local_executor;
mod neovim;
pub mod notify;

pub use background_executor::NeovimBackgroundExecutor;
pub use local_executor::NeovimLocalExecutor;
pub use neovim::Neovim;
#[doc(hidden)]
pub use nvim_oxi as oxi;
#[doc(inline)]
pub use nvimx2_macros::plugin;
