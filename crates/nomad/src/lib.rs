//! # Nomad
//!
//! TODO: docs

#[doc(hidden)]
pub use nvim;
pub use ui;

pub mod action;
pub mod api;
mod apply;
mod autocmd_id;
mod buffer;
mod buffer_id;
mod buffer_snapshot;
mod byte_offset;
mod command;
mod command_args;
mod config;
mod crdt_replacement;
mod edit;
mod editor_id;
mod from_ctx;
pub mod log;
pub mod maybe_future;
pub mod maybe_result;
pub mod module;
mod nomad;
mod nvim_buffer;
mod point;
mod replacement;
pub mod runtime;
mod serde;
pub mod shared;
pub mod streams;
#[cfg(feature = "tests")]
pub mod tests;
mod utils;
pub mod warning;

pub use nomad::Nomad;

pub mod prelude {
    //! TODO: docs

    pub use macros::{async_action, Ready};
    pub use nvim;
    pub use ui::*;

    pub use crate::action::*;
    pub use crate::api::*;
    pub use crate::command_args::*;
    pub use crate::log::*;
    pub use crate::maybe_future::*;
    pub use crate::maybe_result::*;
    pub use crate::module::*;
    pub use crate::runtime::*;
    pub use crate::shared::*;
    pub use crate::streams::*;
    pub use crate::warning::*;
    pub use crate::Nomad;
}

pub use apply::Apply;
pub(crate) use autocmd_id::AutocmdId;
pub use buffer::Buffer;
pub use buffer_id::BufferId;
pub use buffer_snapshot::BufferSnapshot;
pub use byte_offset::ByteOffset;
pub(crate) use command::{Command, ModuleCommands};
pub use crdt_replacement::CrdtReplacement;
pub use edit::Edit;
pub use editor_id::EditorId;
pub use from_ctx::{FromCtx, IntoCtx};
pub use macros::test;
pub use nvim_buffer::{NvimBuffer, NvimBufferDoesntExistError};
pub use point::Point;
pub use replacement::Replacement;
pub use shared::Shared;
