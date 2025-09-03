use auth_types::AuthInfos;
use editor::Context;
use editor::context::Borrowed;
use neovim::Neovim;
use neovim::notify::ContextExt;

use crate::{AuthEditor, github, login, logout};

impl AuthEditor for Neovim {
    type LoginError = github::GitHubLoginError;

    #[allow(clippy::manual_async_fn)]
    fn credential_builder(
        _: &mut Context<Self, Borrowed>,
    ) -> impl Future<Output = Box<keyring::CredentialBuilder>> + Send + 'static
    {
        async move { keyring::default_credential_builder() }
    }

    async fn login(
        ctx: &mut Context<Self>,
    ) -> Result<AuthInfos, Self::LoginError> {
        github::login(ctx).await
    }

    fn on_login_error(
        error: login::LoginError<Self>,
        ctx: &mut Context<Self>,
    ) {
        ctx.notify_error(error);
    }

    fn on_logout_error(error: logout::LogoutError, ctx: &mut Context<Self>) {
        ctx.notify_error(error);
    }
}
