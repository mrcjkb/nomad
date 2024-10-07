//! TODO: docs.

mod api;
mod buffer;
mod command;
mod config;
mod diagnostic;
pub mod events;
mod executor;
mod function;
mod join_handle;
mod module_api;
mod neovim;
mod point;
mod serde;
mod spawner;

pub use api::Api;
pub use buffer::{Buffer, BufferId};
pub use command::{command, Command, CommandArgs, CommandHandle};
pub use diagnostic::{DiagnosticMessage, HighlightGroup};
pub use function::{function, Function, FunctionHandle};
pub use join_handle::NeovimJoinHandle;
pub use module_api::{module_api, ModuleApi};
pub use neovim::Neovim;
pub use point::Point;
pub use spawner::NeovimSpawner;
