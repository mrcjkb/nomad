//! TODO: docs.

mod api;
mod executor;
mod function;
mod join_handle;
mod module_api;
mod neovim;
mod spawner;

pub use api::Api;
pub use function::{function, Function, FunctionEvent, FunctionHandle};
pub use join_handle::NeovimJoinHandle;
pub use module_api::ModuleApi;
pub use neovim::Neovim;
pub use spawner::NeovimSpawner;
