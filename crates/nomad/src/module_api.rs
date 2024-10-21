use core::marker::PhantomData;

use nvim_oxi::Dictionary as NvimDictionary;

use crate::module_commands::ModuleCommands;
use crate::neovim::Autocmd;
use crate::{Command, Function, Module};

/// TODO: docs.
pub struct ModuleApi<M: Module> {
    dictionary: NvimDictionary,
    commands: ModuleCommands,
    ty: PhantomData<M>,
}

impl<M: Module> ModuleApi<M> {
    #[inline]
    pub fn autocmd<T: Autocmd<M>>(self, autocmd: T) -> Self {
        let _ = autocmd.register();
        self
    }

    #[inline]
    pub fn command<T: Command<Module = M>>(mut self, command: T) -> Self {
        self.commands.add_command(command);
        self
    }

    #[inline]
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

    #[inline]
    pub fn new() -> Self {
        Self {
            dictionary: NvimDictionary::default(),
            commands: ModuleCommands::new::<M>(),
            ty: PhantomData,
        }
    }
}
