use core::net::SocketAddr;
use std::path::PathBuf;

use nomad::prelude::WarningMsg;
use serde::Deserialize;
use thiserror::Error as ThisError;
use url::Url;

/// TODO: docs
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct CollabConfig {
    /// TODO: docs
    #[serde(default = "default_enable")]
    enable: bool,

    /// TODO: docs
    #[serde(default = "default_project_dir")]
    project_dir: PathBuf,

    /// TODO: docs
    #[serde(default = "default_server_addr")]
    server_addr: Url,

    /// TODO: docs
    #[serde(default = "default_server_port")]
    server_port: u16,
}

impl Default for CollabConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            project_dir: default_project_dir(),
            server_addr: default_server_addr(),
            server_port: default_server_port(),
        }
    }
}

impl CollabConfig {
    #[allow(dead_code)]
    pub(crate) fn server_addr(&self) -> Result<SocketAddr, InvalidServerAddr> {
        self.server_addr
            .socket_addrs(|| Some(self.server_port))?
            .into_iter()
            .next()
            .ok_or(InvalidServerAddr::EmptyAddresses)
    }
}

fn default_enable() -> bool {
    true
}

fn default_project_dir() -> PathBuf {
    // TODO: this should be a path relative to the `/nomad` path.
    PathBuf::new()
}

fn default_server_addr() -> Url {
    Url::parse("tcp://collab.nomad.foo").expect("address is valid")
}

fn default_server_port() -> u16 {
    64420
}

/// The error type returned by [`CollabConfig::server_addr`].
#[derive(Debug, ThisError)]
pub enum InvalidServerAddr {
    /// The URL resolved to an empty list of socket addresses.
    #[error("URL resolved to an empty list of socket addresses")]
    EmptyAddresses,

    /// The URL is invalid.
    #[error("{0}")]
    InvalidUrl(#[from] std::io::Error),
}

impl From<InvalidServerAddr> for WarningMsg {
    fn from(err: InvalidServerAddr) -> Self {
        let mut msg = WarningMsg::new();
        msg.add("couldn't resolve server address: ");
        msg.add(err.to_string().as_str());
        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the default `CollabConfig` can be created without panicking.
    #[test]
    fn collab_config_default() {
        let _config = CollabConfig::default();
    }

    /// Tests that the server address of the default `CollabConfig` can be
    /// resolved to a valid `SocketAddr`.
    #[test]
    fn collab_config_resolve_server_addr() {
        let _addr = CollabConfig::default().server_addr().unwrap();
    }
}
