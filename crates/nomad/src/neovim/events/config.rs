use core::cmp::Ordering;
use core::marker::PhantomData;

use nvim_oxi::Object as NvimObject;

use crate::neovim::config::{DeserializeConfigError, Setup};
use crate::neovim::diagnostic::DiagnosticSource;
use crate::neovim::serde::deserialize;
use crate::neovim::Neovim;
use crate::{Context, Emitter, Event, Module, Shared};

pub(super) type OnConfigChange =
    Box<dyn Fn(NvimObject) -> Result<(), DeserializeConfigError>>;

/// TODO: docs.
pub struct ConfigEvent<T> {
    pub(in crate::neovim) module_name: &'static str,
    pub(in crate::neovim) buf: Shared<Option<OnConfigChange>>,
    pub(in crate::neovim) ty: PhantomData<T>,
}

impl<T: Module<Neovim>> ConfigEvent<T> {
    pub(in crate::neovim) fn new(buf: Shared<Option<OnConfigChange>>) -> Self {
        Self { module_name: T::NAME.as_str(), buf, ty: PhantomData }
    }
}

impl<T: Module<Neovim>> Event<Neovim> for ConfigEvent<T> {
    type Payload = T::Config;
    type SubscribeCtx = ();

    fn subscribe(
        &mut self,
        emitter: Emitter<Self::Payload>,
        _: &Context<Neovim>,
    ) {
        let on_config_change = Box::new(move |obj| {
            let config = deserialize::<T::Config>(obj).map_err(|err| {
                let mut source = DiagnosticSource::new();
                source
                    .push_segment(Setup::NAME)
                    .push_segment(T::NAME.as_str());
                DeserializeConfigError::new(source, err.into_msg())
            })?;
            emitter.send(config);
            Ok(())
        });

        self.buf.with_mut(|buf| {
            *buf = Some(on_config_change);
        });
    }
}

impl<T> PartialEq for ConfigEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for ConfigEvent<T> {}

impl<T> PartialOrd for ConfigEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for ConfigEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.module_name.cmp(other.module_name)
    }
}
