use alloc::rc::Rc;
use core::cell::RefCell;

use neovim::Ctx;

use crate::prelude::nvim::Dictionary;
use crate::{EnableConfig, Module, ObjectSafeModule};

/// TODO: docs
#[derive(Default)]
pub struct Nomad {
    /// TODO: docs
    api: Dictionary,

    /// TODO: docs
    ctx: Rc<RefCell<Ctx>>,
}

impl Nomad {
    /// TODO: docs
    #[inline]
    pub fn api(self) -> Dictionary {
        self.api
    }

    /// TODO: docs
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    #[inline]
    pub fn with_module<M: Module>(mut self) -> Self {
        let ctx = self.ctx.borrow();

        let init_ctx = ctx.as_init();

        // TODO: docs
        let (config, _set_config) =
            init_ctx.new_input(EnableConfig::<M>::default());

        let module = Rc::new(M::init(config, init_ctx));

        drop(ctx);

        let module_api = ObjectSafeModule::api(&module, &self.ctx);

        self.api.insert(M::NAME.as_str(), module_api);

        for _command in module.commands() {}

        self
    }
}
