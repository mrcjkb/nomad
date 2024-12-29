//! TODO: docs.

mod action;
pub mod api;
mod async_ctx;
mod backend;
mod backend_handle;
pub mod command;
pub mod executor;
mod function;
mod maybe_result;
mod module;
mod neovim_ctx;
pub mod notify;
mod plugin;
mod shared;
mod byte_offset;

pub use byte_offset::ByteOffset;
pub use action::{Action, ActionName};
pub use async_ctx::AsyncCtx;
pub use backend::Backend;
use backend::BackendExt;
use backend_handle::{BackendHandle, BackendMut};
pub use function::Function;
pub use maybe_result::MaybeResult;
pub use module::{Module, ModuleApiCtx, ModuleName};
pub use neovim_ctx::NeovimCtx;
pub use plugin::{Plugin, PluginApiCtx, PluginName};
pub use shared::Shared;
