//! TODO: docs.

pub mod api;
pub mod buffer;
mod convert;
pub mod cursor;
mod events;
pub mod executor;
mod mode;
mod neovim;
pub mod notify;
pub mod selection;
pub mod serde;
#[cfg(feature = "test")]
pub mod tests;
pub mod utils;
pub mod value;

pub use api::NeovimApi;
pub use neovim::Neovim;
#[doc(inline)]
pub use neovim_macros::plugin;
#[doc(inline)]
#[cfg(feature = "test")]
pub use neovim_macros::test;
#[doc(hidden)]
pub use nvim_oxi as oxi;
pub use nvim_oxi::mlua;
