use core::future::Future;
use core::pin::Pin;

use crate::config::Setup;
use crate::diagnostics::{DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::neovim::{Api, Neovim};
use crate::{Context, JoinHandle, Module, Spawner};

/// TODO: docs.
pub struct Nomad {
    api: Api,
    ctx: Context<Neovim>,
    run: Vec<Pin<Box<dyn Future<Output = ()>>>>,
    setup: Setup,
}

impl Nomad {
    pub(crate) const COMMAND_NAME: &'static str = "Mad";

    /// TODO: docs.
    pub fn into_api(self) -> Api {
        self.api
    }

    /// TODO: docs.
    pub fn new(neovim: Neovim) -> Self {
        Self {
            api: Api::default(),
            ctx: Context::new(neovim),
            run: Vec::default(),
            setup: Setup::default(),
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
        let config_rx = self.setup.add_module::<M>();
        let module = M::from(config_rx);
        let module_api = module.init();
        self.api += module_api;
        self.run.push({
            Box::pin(async move {
                if let Err(err) = module.run().await.into_result() {
                    let mut source = DiagnosticSource::new();
                    source.push_segment(M::NAME.as_str());
                    err.into().emit(Level::Error, source);
                }
            })
        });
        self
    }

    pub(crate) fn log_dir(&self) -> collab_fs::AbsUtf8PathBuf {
        todo!();
    }
}
