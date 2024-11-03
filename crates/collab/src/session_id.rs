use core::num::ParseIntError;
use core::{fmt, str};

use nomad::diagnostics::DiagnosticMessage;
use nomad::CommandArgs;

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct SessionId(collab_server::SessionId);

impl SessionId {
    pub(crate) fn into_inner(self) -> collab_server::SessionId {
        self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0.into_u64())
    }
}

impl str::FromStr for SessionId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u64::from_str_radix(s, 16).map(collab_server::SessionId::new).map(Self)
    }
}

impl TryFrom<&mut CommandArgs> for SessionId {
    type Error = DiagnosticMessage;

    fn try_from(args: &mut CommandArgs) -> Result<Self, Self::Error> {
        let [id] = <&[String; 1]>::try_from(args)?;
        id.parse::<Self>().map_err(|err| {
            let mut msg = DiagnosticMessage::new();
            msg.push_str(err.to_string());
            msg
        })
    }
}
