//! TODO: docs.

use auth_types::{AuthInfos, OAuthState};
use editor::{Access, Context, Editor};
use rand::Rng;
use url::Url;

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub(crate) struct GitHubConfig {
    /// The base URL to send the login request to.
    pub login_request_url: Url,
}

pub(crate) async fn login<Ed: Editor>(
    _config: impl Access<GitHubConfig>,
    ctx: &mut Context<Ed>,
) -> Result<AuthInfos, GitHubLoginError> {
    let _state = OAuthState::from_bytes(ctx.with_rng(Rng::random));

    todo!();
}

/// TODO: docs.
#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[display("{_0}")]
pub enum GitHubLoginError {}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            login_request_url: Url::parse("https://auth.collab.foo")
                .expect("valid URL"),
        }
    }
}
