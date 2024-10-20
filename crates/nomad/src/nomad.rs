use core::future::Future;
use core::pin::Pin;

use crate::neovim::{Api, Neovim};
use crate::{Context, Editor, JoinHandle, Module, Spawner};

/// TODO: docs.
pub struct Nomad {
    api: Api,
    ctx: Context<Neovim>,
    run: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Nomad {
    /// TODO: docs.
    pub fn into_api(self) -> Api {
        self.api
    }

    /// TODO: docs.
    pub fn new(neovim: Neovim) -> Self {
        crate::log::init(&neovim.log_dir());
        Self {
            api: Api::default(),
            ctx: Context::new(neovim),
            run: Vec::default(),
        }
    }

    /// TODO: docs.
    pub fn start_modules(&mut self) {
        for fut in self.run.drain(..) {
            self.ctx.spawner().spawn(fut).detach();
        }
    }

    /// TODO: docs.
    #[track_caller]
    pub fn with_module<M: Module>(mut self) -> Self {
        let (mut module, module_api) = M::init(&self.ctx);
        self.api += module_api;
        self.run.push({
            let ctx = self.ctx.clone();
            Box::pin(async move { module.run(&ctx).await })
        });
        self
    }
}
