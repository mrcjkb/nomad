/// The [`Params`](collab_server::Params) used by the Collab server deployed at
/// `collab.nomad.foo`.
pub struct NomadParams;

impl collab_server::Params for NomadParams {
    const MAX_FRAME_LEN: u32 = 32 * 1024; // 32 KiB

    type AuthenticateInfos = crate::AuthInfos;
    type AuthenticateError = crate::AuthError;
    type SessionId = crate::SessionId;
}
