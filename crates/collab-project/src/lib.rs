//! TODO: docs.

mod annotation;
pub mod binary;
pub mod fs;
mod project;
mod project_builder;
pub mod symlink;
pub mod text;

pub use collab_types::puff::abs_path;
pub use project::{DecodeError, LocalPeerIsNotOwnerError, Project};
pub use project_builder::ProjectBuilder;
