use nomad::prelude::*;

use crate::{Config, Start};

/// TODO: docs.
pub struct Collab {
    _config: Get<Config>,
}

impl Collab {
    fn new(config: Get<Config>) -> Self {
        Self { _config: config }
    }
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;

    fn init(config: Get<Self::Config>) -> Api<Self> {
        Api::new(Self::new(config.clone())).with_command(Start::new(config))
    }

    async fn run(&self) -> impl MaybeResult<()> {}
}
