use crate::github;

/// The configuration of the [`Auth`](crate::Auth) module.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[cfg(feature = "neovim")]
    #[serde(flatten)]
    pub(crate) github: github::GitHubConfig,
}
