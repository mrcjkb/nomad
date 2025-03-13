use collab_server::message::GitHubHandle;
use collab_server::nomad::NomadAuthenticateInfos;

/// TODO: docs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AuthInfos {
    inner: NomadAuthenticateInfos,
}

impl AuthInfos {
    /// TODO: docs.
    pub fn handle(&self) -> &GitHubHandle {
        &self.inner.github_handle
    }

    #[cfg(any(test, feature = "mock"))]
    #[track_caller]
    pub(crate) fn dummy<Gh>(github_handle: Gh) -> Self
    where
        Gh: TryInto<collab_server::message::GitHubHandle>,
        Gh::Error: core::fmt::Debug,
    {
        Self {
            inner: NomadAuthenticateInfos {
                github_handle: github_handle
                    .try_into()
                    .expect("invalid github handle"),
            },
        }
    }
}

impl AsRef<GitHubHandle> for AuthInfos {
    fn as_ref(&self) -> &GitHubHandle {
        self.handle()
    }
}

impl From<AuthInfos> for NomadAuthenticateInfos {
    fn from(infos: AuthInfos) -> Self {
        infos.inner
    }
}
