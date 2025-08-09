use crate::GitHubHandle;

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuthInfos {
    /// TODO: docs.
    pub github_handle: GitHubHandle,
}

impl AuthInfos {
    /// TODO: docs.
    #[cfg(any(test, feature = "mock"))]
    #[track_caller]
    pub fn dummy<Gh>(github_handle: Gh) -> Self
    where
        Gh: TryInto<GitHubHandle, Error: core::fmt::Debug>,
    {
        Self {
            github_handle: github_handle
                .try_into()
                .expect("invalid github handle"),
        }
    }
}

impl AsRef<GitHubHandle> for AuthInfos {
    #[inline]
    fn as_ref(&self) -> &GitHubHandle {
        &self.github_handle
    }
}
