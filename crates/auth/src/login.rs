//! TODO: docs.

use auth_types::PeerHandle;
use editor::command::ToCompletionFn;
use editor::module::AsyncAction;
use editor::{Access, Context, Shared};

use crate::auth_state::AuthInfos;
use crate::credential_store::{self, CredentialStore};
use crate::{Auth, AuthEditor, AuthState, Config};

/// TODO: docs.
#[derive(Clone, Default)]
pub struct Login {
    config: Shared<Config>,
    credential_store: CredentialStore,
    state: AuthState,
}

impl Login {
    pub(crate) async fn call_inner<Ed: AuthEditor>(
        &self,
        ctx: &mut Context<Ed>,
    ) -> Result<(), LoginError<Ed>> {
        if let Some(peer_handle) = self.state.with(|maybe_infos| {
            maybe_infos.as_ref().map(|infos| infos.peer_handle.clone())
        }) {
            return Err(LoginError::AlreadyLoggedIn(peer_handle));
        }

        let (access_token, peer_handle) = Ed::login(self.config.clone(), ctx)
            .await
            .map_err(LoginError::Login)?;

        let auth_infos = AuthInfos { access_token, peer_handle };

        self.state.set_logged_in(auth_infos.clone());

        // Persisting the credentials blocks, so do it in the background.
        let credential_store = self.credential_store.clone();
        ctx.spawn_background(async move {
            credential_store.persist(auth_infos).await
        })
        .await
        .map_err(Into::into)
    }
}

impl<Ed: AuthEditor> AsyncAction<Ed> for Login {
    const NAME: &str = "login";

    type Args = ();

    async fn call(&mut self, _: Self::Args, ctx: &mut Context<Ed>) {
        if let Err(err) = self.call_inner(ctx).await {
            Ed::on_login_error(err, ctx);
        }
    }
}

/// TODO: docs.
#[derive(cauchy::Debug, derive_more::Display, cauchy::Error)]
pub enum LoginError<Ed: AuthEditor> {
    /// TODO: docs.
    #[display("Already logged in as {_0}")]
    AlreadyLoggedIn(PeerHandle),

    /// TODO: docs.
    #[display("Couldn't get credentials from keyring: {_0}")]
    GetCredential(keyring::Error),

    /// TODO: docs.
    #[display("{_0}")]
    Login(Ed::LoginError),

    /// TODO: docs.
    #[display("Couldn't persist credentials: {_0}")]
    PersistCredentials(keyring::Error),
}

impl From<&Auth> for Login {
    fn from(auth: &Auth) -> Self {
        Self {
            config: auth.config.clone(),
            credential_store: auth.credential_store.clone(),
            state: auth.state(),
        }
    }
}

impl<Ed: AuthEditor> ToCompletionFn<Ed> for Login {
    fn to_completion_fn(&self) {}
}

impl<Ed: AuthEditor> From<credential_store::Error> for LoginError<Ed> {
    fn from(err: credential_store::Error) -> Self {
        use credential_store::Error::*;
        match err {
            GetCredential(err) => Self::GetCredential(err),
            Op(err) => Self::PersistCredentials(err),
        }
    }
}
