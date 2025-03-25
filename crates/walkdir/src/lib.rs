//! TODO: docs.

mod filter;
#[cfg(feature = "gitignore")]
mod gitignore;
mod walkdir;

pub use filter::{And, Either, Filter, Filtered};
#[cfg(feature = "gitignore")]
pub use gitignore::GitIgnore;
pub use walkdir::{WalkDir, WalkError};
