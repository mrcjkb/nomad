//! TODO: docs.

mod server_address;

use ed::fs::AbsPathBuf;
pub(crate) use server_address::ServerAddress;

/// TODO: docs.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The address of the server to connect to when starting or joining an
    /// editing session.
    pub(crate) server_address: ServerAddress,

    /// TODO: docs.
    pub(crate) store_remote_projects_under: Option<AbsPathBuf>,
}
