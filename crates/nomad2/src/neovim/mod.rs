//! TODO: docs.

mod executor;
mod function;
mod join_handle;
mod module_api;
mod neovim;
mod spawner;

pub use function::{function, Function, FunctionEvent, FunctionHandle};
pub use join_handle::NeovimJoinHandle;
pub use module_api::NeovimModuleApi;
pub use neovim::Neovim;
pub use spawner::NeovimSpawner;
