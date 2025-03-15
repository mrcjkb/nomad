use std::sync::{Arc, OnceLock};

use crate::AuthInfos;
use crate::async_once_lock::AsyncOnceLock;

#[derive(Clone, Default)]
pub(crate) struct CredentialStore {
    entry: Arc<OnceLock<keyring::Entry>>,
    builder: Arc<AsyncOnceLock<Box<keyring::CredentialBuilder>>>,
}

pub(crate) enum Error {
    GetCredential(keyring::Error),
    Op(keyring::Error),
}

impl CredentialStore {
    const APP_NAME: &str = "nomad";
    const SECRET_NAME: &str = "auth-infos";

    pub(crate) async fn delete(&self) -> Result<(), Error> {
        self.entry().await?.delete_credential().map_err(Error::Op)
    }

    pub(crate) async fn persist(&self, infos: AuthInfos) -> Result<(), Error> {
        let entry = self.entry().await?;
        let json = match serde_json::to_string(&infos) {
            Ok(json) => json,
            Err(_) => unreachable!("Serialize impl never fails"),
        };
        entry.set_password(&json).map_err(Error::Op)
    }

    pub(crate) async fn retrieve(&self) -> Result<Option<AuthInfos>, Error> {
        match self.entry().await?.get_password() {
            Ok(json) => Ok(serde_json::from_str(&json).ok()),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(err) => Err(Error::Op(err)),
        }
    }

    pub(crate) fn set_builder(
        &self,
        builder: Box<keyring::CredentialBuilder>,
    ) {
        let _ = self.builder.set(builder);
    }

    async fn entry(&self) -> Result<&keyring::Entry, Error> {
        match &self.entry.get() {
            Some(entry) => Ok(entry),
            None => {
                let entry = self
                    .builder
                    .wait()
                    .await
                    .build(None, Self::APP_NAME, Self::SECRET_NAME)
                    .map(keyring::Entry::new_with_credential)
                    .map_err(Error::GetCredential)?;
                Ok(self.entry.get_or_init(|| entry))
            },
        }
    }
}
