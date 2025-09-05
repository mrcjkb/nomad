//! TODO: docs.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod async_once_lock;
mod auth;
mod config;
mod credential_store;
mod editors;
#[cfg(feature = "neovim")]
pub mod github;
pub mod login;
pub mod logout;

pub use auth::Auth;
#[doc(inline)]
pub use auth_types::{AuthError, AuthInfos, GitHubHandle};
pub use config::Config;
pub use editors::AuthEditor;
#[cfg(feature = "mock")]
pub use editors::mock;
