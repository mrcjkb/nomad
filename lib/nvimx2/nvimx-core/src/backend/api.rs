//! TODO: docs.

use crate::ByteOffset;
use crate::backend::Backend;
use crate::command::{CommandArgs, CommandCompletion};
use crate::module::Module;
use crate::notify::Name;
use crate::plugin::Plugin;

/// TODO: docs.
pub trait Api<P, B>: 'static + Sized
where
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    type ModuleApi<'a, M: Module<P, B>>: ModuleApi<Self, P, M, B>;

    /// TODO: docs.
    fn add_command<Command, CompletionFn, Completions>(
        &mut self,
        command: Command,
        completion_fn: CompletionFn,
    ) where
        Command: FnMut(CommandArgs) + 'static,
        CompletionFn: FnMut(CommandArgs, ByteOffset) -> Completions + 'static,
        Completions: IntoIterator<Item = CommandCompletion>;

    /// TODO: docs.
    fn as_module(&mut self) -> Self::ModuleApi<'_, P>;
}

/// TODO: docs.
pub trait ModuleApi<A, P, M, B>: Sized
where
    A: Api<P, B>,
    P: Plugin<B>,
    M: Module<P, B>,
    B: Backend,
{
    /// TODO: docs.
    fn add_constant(&mut self, constant_name: Name, value: B::ApiValue);

    /// TODO: docs.
    fn add_function<Fun>(&mut self, fun_name: Name, fun: Fun)
    where
        Fun: FnMut(B::ApiValue) -> Option<B::ApiValue> + 'static;

    /// TODO: docs.
    fn as_submodule<M2>(&mut self) -> A::ModuleApi<'_, M2>
    where
        M2: Module<P, B>;

    /// TODO: docs.
    fn finish(self);
}
