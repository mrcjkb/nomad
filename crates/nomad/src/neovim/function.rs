use core::marker::PhantomData;

use nvim_oxi::{Function as NvimFunction, Object as NvimObject};
use serde::de::DeserializeOwned;

use super::events::FunctionEvent;
use super::Neovim;
use crate::{Context, Module, Shared, Subscription};

/// TODO: docs.
pub fn function<T: Function>(
    ctx: &Context<Neovim>,
) -> (FunctionHandle, Subscription<FunctionEvent<T>, Neovim>) {
    let buf = Shared::new(None);
    let event = FunctionEvent {
        module_name: T::Module::NAME.as_str(),
        function_name: T::NAME,
        function_buf: buf.clone(),
        ty: PhantomData,
    };
    let sub = ctx.subscribe(event);
    let handle = FunctionHandle {
        name: T::NAME,
        module_name: T::Module::NAME.as_str(),
        inner: buf.with_mut(Option::take).expect("just set when subscribing"),
    };
    (handle, sub)
}

/// TODO: docs.
pub trait Function: 'static {
    /// TODO: docs.
    const NAME: &'static str;

    /// TODO: docs.
    type Args: Clone + DeserializeOwned;

    /// TODO: docs.
    type Module: Module<Neovim>;
}

/// TODO: docs.
pub struct FunctionHandle {
    pub(super) name: &'static str,
    pub(super) module_name: &'static str,
    pub(super) inner: NvimFunction<NvimObject, ()>,
}
