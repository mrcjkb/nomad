use auth_types::{AccessToken, PeerHandle};
use editor::{Access, Shared};

/// TODO: docs.
#[derive(Clone, Default)]
pub struct AuthState {
    inner: Shared<Option<AuthInfos>>,
}

impl AuthState {
    pub(crate) fn set_logged_in(&self, infos: AuthInfos) {
        self.inner.set(Some(infos));
    }

    /// Sets the state to logged out, returning whether it was logged in
    /// before.
    pub(crate) fn set_logged_out(&self) -> bool {
        self.inner.take().is_some()
    }
}

/// TODO: docs.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthInfos {
    /// TODO: docs.
    pub access_token: AccessToken,

    /// TODO: docs.
    pub peer_handle: PeerHandle,
}

impl Access<Option<AuthInfos>> for AuthState {
    fn with<R>(&self, fun: impl FnOnce(&Option<AuthInfos>) -> R) -> R {
        self.inner.with(fun)
    }
}

impl From<AuthInfos> for AccessToken {
    fn from(auth_infos: AuthInfos) -> Self {
        auth_infos.access_token
    }
}

impl From<AuthInfos> for PeerHandle {
    fn from(auth_infos: AuthInfos) -> Self {
        auth_infos.peer_handle
    }
}
