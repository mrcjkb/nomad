//! TODO: docs.

#![feature(min_specialization)]

pub mod backend;
mod collab;
mod config;
mod leave;
mod session;
mod sessions;
pub mod start;
mod yank;

pub use backend::CollabBackend;
pub use collab::Collab;
