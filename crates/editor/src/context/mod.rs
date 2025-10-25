//! TODO: docs.

mod buffer;
mod context;
mod cursor;
mod event_handle;
mod selection;
mod state;

pub use buffer::Buffer;
pub use context::{
    BorrowState,
    Borrowed,
    BorrowedInner,
    Context,
    NotBorrowed,
};
pub use cursor::Cursor;
pub use event_handle::EventHandle;
pub use selection::Selection;
pub(crate) use state::{State, StateHandle, StateMut};
