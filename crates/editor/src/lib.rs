//! TODO: docs.

#![feature(min_specialization)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod access;
pub mod command;
pub mod context;
mod editor;
pub mod module;
pub mod notify;
pub mod shared;
mod util;

pub use access::{Access, AccessMut};
pub use context::Context;
pub use editor::*;
pub use shared::Shared;
