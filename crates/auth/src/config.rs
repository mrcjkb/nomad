use url::Url;

/// The configuration of the [`Auth`](crate::Auth) module.
#[derive(Debug, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The URL where the authentication server is running.
    pub(crate) server_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_url: Url::parse("https://auth.collab.foo")
                .expect("valid URL"),
        }
    }
}
