use std::sync::{Arc, OnceLock};

use auth_types::JsonWebToken;

use crate::async_once_lock::AsyncOnceLock;

#[derive(Clone, Default)]
pub(crate) struct CredentialStore {
    entry: Arc<OnceLock<keyring::Entry>>,
    builder: Arc<AsyncOnceLock<Box<keyring::CredentialBuilder>>>,
}

#[derive(Debug, derive_more::Display, cauchy::From)]
#[display("{_0}")]
pub(crate) enum Error {
    GetCredential(#[from] auth_types::jsonwebtoken::errors::Error),
    Op(#[from] keyring::Error),
}

impl CredentialStore {
    const APP_NAME: &str = "nomad";
    const SECRET_NAME: &str = "auth-infos";

    pub(crate) async fn delete(&self) -> Result<(), keyring::Error> {
        self.entry().await?.delete_credential()
    }

    pub(crate) async fn persist(
        &self,
        jwt: &JsonWebToken,
    ) -> Result<(), keyring::Error> {
        self.entry().await?.set_password(jwt.as_str())
    }

    pub(crate) async fn retrieve(
        &self,
    ) -> Result<Option<JsonWebToken>, Error> {
        match self.entry().await?.get_password() {
            Ok(jwt) => JsonWebToken::from_str_on_client(&jwt)
                .map(Some)
                .map_err(Into::into),

            Err(keyring::Error::NoEntry) => Ok(None),

            // TODO: upstream this.
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "openbsd"
            ))]
            Err(keyring::Error::NoStorageAccess(err)) => {
                if let Some(dbus_secret_service::Error::NoResult) =
                    err.downcast_ref()
                {
                    Ok(None)
                } else {
                    Err(keyring::Error::NoStorageAccess(err).into())
                }
            },

            Err(err) => Err(err.into()),
        }
    }

    pub(crate) fn set_builder(
        &self,
        builder: Box<keyring::CredentialBuilder>,
    ) {
        let _ = self.builder.set(builder);
    }

    async fn entry(&self) -> Result<&keyring::Entry, keyring::Error> {
        match &self.entry.get() {
            Some(entry) => Ok(entry),
            None => {
                let entry = self
                    .builder
                    .wait()
                    .await
                    .build(None, Self::APP_NAME, Self::SECRET_NAME)
                    .map(keyring::Entry::new_with_credential)?;
                Ok(self.entry.get_or_init(|| entry))
            },
        }
    }
}
