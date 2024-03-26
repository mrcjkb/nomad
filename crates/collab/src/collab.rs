use nomad::prelude::*;

use crate::{Config, Context, Join, Start};

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
        let ctx = Context::new(config.clone());

        let join = Join::new(&ctx);

        let start = Start::new(&ctx);

        Api::new(Self::new(config.clone()))
            .with_command(start.clone())
            .with_command(join.clone())
            .with_function(start)
            .with_function(join)
    }

    async fn run(&self) {}
}
