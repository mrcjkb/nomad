//! TODO: docs.

mod async_once_lock;
mod auth;
mod credential_store;
mod editors;
pub mod github;
pub mod login;
pub mod logout;

pub use auth::Auth;
#[doc(inline)]
pub use auth_types::{AuthError, AuthInfos, GitHubHandle};
pub use editors::AuthEditor;
#[cfg(feature = "mock")]
pub use editors::mock;
