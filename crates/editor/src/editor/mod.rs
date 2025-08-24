//! TODO: docs.

mod agent_id;
mod api;
mod buffer;
mod cursor;
mod editor;
mod editor_adapter;
mod selection;

pub use agent_id::AgentId;
pub use api::{Api, ApiValue, Key, MapAccess, Value};
pub use buffer::{Buffer, Chunks, Edit, Replacement};
pub use cursor::Cursor;
pub use editor::Editor;
pub use editor_adapter::EditorAdapter;
pub use selection::Selection;

/// A byte offset in a buffer.
pub type ByteOffset = usize;
