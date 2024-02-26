//! # Neovim
//!
//! TODO: docs

mod ctx;
mod get;
mod neovim;
mod set;

pub use ctx::{GetCtx, InitCtx, SetCtx};
pub use get::Get;
pub use neovim::Neovim;
pub use set::Set;
