//! TODO: docs.

pub mod api;
mod background_executor;
mod buffer;
mod convert;
mod local_executor;
mod neovim;
pub mod notify;
pub mod serde;
pub mod value;

pub use api::NeovimApi;
pub mod executor {
    //! TODO: docs.
    pub use crate::background_executor::NeovimBackgroundExecutor;
    pub use crate::local_executor::NeovimLocalExecutor;
}
pub use buffer::NeovimBuffer;
pub use neovim::Neovim;
#[doc(hidden)]
pub use nvim_oxi as oxi;
#[cfg(feature = "mlua")]
pub use nvim_oxi::mlua;
#[doc(inline)]
pub use nvimx2_macros::plugin;

/// TODO: docs.
pub type NeovimFs = nvimx_core::fs::os::OsFs;
