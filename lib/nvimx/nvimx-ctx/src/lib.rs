//! TODO: docs.

mod actor_id;
mod actor_map;
mod autocmd;
mod autocmd_ctx;
mod boo;
mod buffer_ctx;
mod buffer_id;
mod decoration_provider;
mod file_ctx;
mod neovim_ctx;
mod on_bytes;
mod pane_ctx;
mod pane_id;
mod text_buffer_ctx;
mod text_file_ctx;

pub use actor_id::ActorId;
pub use autocmd::{AutoCommand, AutoCommandEvent, ShouldDetach};
pub use autocmd_ctx::AutoCommandCtx;
pub use buffer_ctx::BufferCtx;
pub use buffer_id::BufferId;
pub use decoration_provider::Selection;
pub use file_ctx::FileCtx;
pub use neovim_ctx::NeovimCtx;
pub use on_bytes::{OnBytesArgs, RegisterOnBytesArgs};
pub use pane_ctx::PaneCtx;
pub use pane_id::PaneId;
pub use text_buffer_ctx::TextBufferCtx;
pub use text_file_ctx::TextFileCtx;
