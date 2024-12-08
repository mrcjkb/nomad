use crate::server_socket::ServerSocket;

#[derive(Default, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// TODO: docs.
    pub(crate) server_socket: ServerSocket,
}
