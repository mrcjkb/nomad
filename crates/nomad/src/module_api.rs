use core::marker::PhantomData;

use nvim_oxi::Dictionary as NvimDictionary;

use crate::module_commands::ModuleCommands;
use crate::{Autocmd, Command, Function, Module};

/// TODO: docs.
pub struct ModuleApi<M: Module> {
    pub(crate) dictionary: NvimDictionary,
    pub(crate) commands: ModuleCommands,
    ty: PhantomData<M>,
}

impl<M: Module> ModuleApi<M> {
    pub fn autocmd<T: Autocmd>(self, autocmd: T) -> Self {
        let _ = autocmd.register();
        self
    }

    pub fn command<T: Command<Module = M>>(mut self, command: T) -> Self {
        self.commands.add_command(command);
        self
    }

    pub fn function<T: Function<Module = M>>(mut self, function: T) -> Self {
        if self.dictionary.get(T::NAME.as_str()).is_some() {
            panic!(
                "a function with the name '{}' has already been added to the \
                 API for module '{}'",
                T::NAME,
                M::NAME,
            );
        }
        self.dictionary.insert(T::NAME.as_str(), function.into_function());
        self
    }

    pub fn new() -> Self {
        Self {
            dictionary: NvimDictionary::default(),
            commands: ModuleCommands::new::<M>(),
            ty: PhantomData,
        }
    }
}
