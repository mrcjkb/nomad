//! TODO: docs.

#![feature(min_specialization)]

mod backend;
mod collab;
mod config;
mod session;
mod start;

pub use backend::CollabBackend;
pub use collab::Collab;
