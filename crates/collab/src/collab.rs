use core::time::Duration;

use nomad::prelude::*;

use crate::CollabConfig;

/// TODO: docs.
pub struct Collab {
    config: Get<CollabConfig>,
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = CollabConfig;

    #[inline]
    fn init(config: Get<Self::Config>, ctx: &InitCtx) -> Api<Self> {
        let (counter, set_counter) = ctx.new_input(0u64);

        let increment = Increment { set_counter };

        let print = Print { counter };

        Api::new(Self { config })
            .with_command(increment.clone())
            .with_command(print.clone())
            .with_function(increment)
            .with_function(print)
    }

    #[inline]
    async fn run(
        &self,
        // _ctx: &mut SetCtx,
    ) -> impl MaybeResult<()> {
        let mut count = 0;

        loop {
            nvim::print!("{}'s count is {count}", Self::NAME);
            sleep(Duration::from_secs(1)).await;
            count += 1;
            break;
        }
    }
}

#[derive(Clone)]
struct Print {
    counter: Get<u64>,
}

impl Action<Collab> for Print {
    const NAME: ActionName = action_name!("print");

    type Args = ();

    #[inline]
    fn execute(&self, _args: (), ctx: &mut SetCtx) {
        nvim::print!("Collab counter is now {:?}", self.counter.get(ctx))
    }
}

#[derive(Clone)]
struct Increment {
    set_counter: Set<u64>,
}

impl Action<Collab> for Increment {
    const NAME: ActionName = action_name!("increment");

    type Args = ();

    #[inline]
    fn execute(&self, _args: (), ctx: &mut SetCtx) {
        self.set_counter.update(|counter| *counter += 1, ctx)
    }
}
