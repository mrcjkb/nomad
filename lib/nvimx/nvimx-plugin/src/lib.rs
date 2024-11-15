//! TODO: docs.

mod action;
mod action_name;
mod async_action;
mod command;
mod config;
mod function;
mod log;
mod module;
mod module_api;
mod module_name;
mod module_subcommands;
mod plugin;
mod plugin_ctx;
mod serde;
mod subcommand;
mod subcommand_args;

pub use action::Action;
pub use action_name::ActionName;
pub use async_action::AsyncAction;
pub use config::ConfigReceiver;
pub use function::Function;
pub use module::Module;
pub use module_api::ModuleApi;
pub use module_name::ModuleName;
pub use nvimx_plugin_macros::{action_name, module_name};
pub use plugin::Plugin;
pub use plugin_ctx::PluginCtx;
pub use subcommand::SubCommand;
