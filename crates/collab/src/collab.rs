use nomad::prelude::*;

use crate::{Activity, Config, Join, Start};

/// TODO: docs.
pub struct Collab {
    pub(crate) activity: Shared<Activity>,
    pub(crate) config: Get<Config>,
    pub(crate) _ctx: Ctx,
}

impl Collab {
    fn new(config: Get<Config>, ctx: Ctx) -> Self {
        Self { activity: Shared::new(Activity::default()), config, _ctx: ctx }
    }
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;

    fn init(config: Get<Self::Config>, ctx: &Ctx) -> Api<Self> {
        let collab = Self::new(config, ctx.clone());

        let join = Join::new(&collab);

        let start = Start::new(&collab);

        Api::new(collab)
            .with_command(start.clone())
            .with_command(join.clone())
            .with_function(start)
            .with_function(join)
    }

    async fn run(&self) {}
}
