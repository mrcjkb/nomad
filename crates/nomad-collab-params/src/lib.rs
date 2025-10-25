//! This crate contains the [`Params`][NomadParams] used by Nomad's collab
//! server running at `collab.nomad.foo`.

mod auth_error;
mod auth_infos;
mod nomad_params;
mod session_id;

pub use auth_error::AuthError;
pub use auth_infos::AuthInfos;
pub use auth_types;
pub use nomad_params::NomadParams;
pub use session_id::SessionId;

/// TODO: docs.
pub const API_VERSION: u32 = 4;
