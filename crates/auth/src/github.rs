//! TODO: docs.

use auth_types::{AuthInfos, OAuthState};
use editor::{Context, Editor};
use rand::Rng;

pub(crate) async fn login<Ed: Editor>(
    ctx: &mut Context<Ed>,
) -> Result<AuthInfos, GitHubLoginError> {
    let _state = OAuthState::from_bytes(ctx.with_rng(Rng::random));
    todo!();
}

/// TODO: docs.
#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[display("{_0}")]
pub enum GitHubLoginError {}
