use core::cmp::Ordering;
use core::marker::PhantomData;

use nvim_oxi::{Function as NvimFunction, Object as NvimObject};

use crate::neovim::diagnostic::{DiagnosticSource, Level};
use crate::neovim::serde::deserialize;
use crate::neovim::{Function, Neovim};
use crate::{Context, Emitter, Event, Module, Shared};

/// TODO: docs.
pub struct FunctionEvent<T> {
    pub(in crate::neovim) module_name: &'static str,
    pub(in crate::neovim) function_name: &'static str,
    pub(in crate::neovim) function_buf:
        Shared<Option<NvimFunction<NvimObject, ()>>>,
    pub(in crate::neovim) ty: PhantomData<T>,
}

impl<T: Function> Event<Neovim> for FunctionEvent<T> {
    type Payload = T::Args;
    type SubscribeCtx = ();

    fn subscribe(&mut self, emitter: Emitter<T::Args>, _: &Context<Neovim>) {
        let nvim_fun = NvimFunction::<NvimObject, ()>::from_fn(move |obj| {
            match deserialize::<T::Args>(obj) {
                Ok(payload) => emitter.send(payload),
                Err(err) => {
                    let mut source = DiagnosticSource::new();
                    source
                        .push_segment(T::Module::NAME.as_str())
                        .push_segment(T::NAME);
                    err.into_msg().emit(Level::Error, source);
                },
            };
        });

        self.function_buf.with_mut(|buf| {
            *buf = Some(nvim_fun);
        });
    }
}

impl<T> PartialEq for FunctionEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for FunctionEvent<T> {}

impl<T> PartialOrd for FunctionEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for FunctionEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.module_name.cmp(other.module_name) {
            Ordering::Equal => self.function_name.cmp(other.function_name),
            ord => ord,
        }
    }
}
