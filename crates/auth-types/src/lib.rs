//! TODO: docs.

mod access_token;
mod github_access_token;
mod github_client_id;
mod oauth_state;

pub use access_token::AccessToken;
pub use github_access_token::GitHubAccessToken;
pub use github_client_id::GitHubClientId;
pub use oauth_state::{OAuthState, OAuthStateFromStrError};

/// The [`GitHubClientId`] assigned to Nomad.
pub const NOMAD_GITHUB_CLIENT_ID: GitHubClientId =
    GitHubClientId("Iv23liZkCzK2uYG2jbkh");
