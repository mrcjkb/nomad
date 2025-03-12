//! TODO: docs.

#![feature(min_specialization)]
#![feature(precise_capturing_in_traits)]

mod backend;
mod collab;
pub mod config;
pub mod join;
pub mod leave;
pub mod project;
mod root_markers;
mod session;
pub mod start;
pub mod yank;

pub use backend::CollabBackend;
#[cfg(feature = "mock")]
pub use backend::mock;
pub use collab::Collab;
