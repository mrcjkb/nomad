/// Authentication information for connecting to the Nomad collaboration
/// server.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthInfos {
    /// The API version the client is using.
    pub api_version: u32,

    /// The JSON Web Token provided by the auth server.
    pub jwt: String,
}

impl From<auth_types::JsonWebToken> for AuthInfos {
    fn from(jwt: auth_types::JsonWebToken) -> Self {
        Self { api_version: crate::API_VERSION, jwt: jwt.as_str().to_owned() }
    }
}
