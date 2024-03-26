use nomad::prelude::{new_input, Get, Set};

use crate::{Config, SessionState};

/// TODO: docs
pub(crate) struct Context {
    pub(crate) config: Get<Config>,
    pub(crate) set_state: Set<SessionState>,
    pub(crate) state: Get<SessionState>,
}

impl Context {
    pub(crate) fn new(config: Get<Config>) -> Self {
        let (state, set_state) = new_input(SessionState::default());
        Self { config, set_state, state }
    }
}
