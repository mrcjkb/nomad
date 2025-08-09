//! TODO: docs.

use crate::Protocol;

/// The [`Protocol`] used by the Collab server deployed at `collab.nomad.foo`.
pub struct NomadProtocol;

impl Protocol for NomadProtocol {
    const MAX_FRAGMENT_LEN: u32 = 2048;

    type AuthenticateInfos = auth_types::AuthInfos;
    type AuthenticateError = auth_types::AuthError;
    type SessionId = ulid::Ulid;
}
