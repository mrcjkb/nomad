use super::ModuleApi;
use crate::{Backend, Function, Module, notify};

/// TODO: docs.
pub trait ModuleApiBuilder<MA: ModuleApi<M, B>, M: Module<B>, B: Backend> {
    /// TODO: docs.
    fn build(self) -> MA;

    /// TODO: docs.
    fn add_function<Fun, Cb, Err>(&mut self, callback: Cb)
    where
        Fun: Function<B, Module = M>,
        Cb: FnMut(Fun::Args) -> Result<Fun::Return, Err> + 'static,
        Err: notify::Error;
}
