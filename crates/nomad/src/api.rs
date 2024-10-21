use fxhash::FxHashMap;
use nvim_oxi::{lua, Dictionary as NvimDictionary};

use crate::module_api::{ModuleApi, ModuleCommands};
use crate::{Module, ModuleName};

pub(crate) struct Api {
    /// A dictionary from [`ModuleName`] to the corresponding module's API
    /// dictionary.
    dictionary: NvimDictionary,

    /// A map from module name to the commands for that module.
    commands: FxHashMap<ModuleName, ModuleCommands>,
}

impl Api {
    pub(crate) fn add_module<M: Module>(&mut self, module_api: ModuleApi<M>) {
        if self.commands.contains_key(&M::NAME) {
            panic!(
                "a module with the name '{}' has already been added to the \
                 API",
                M::NAME.as_str()
            );
        }
        self.commands.insert(M::NAME, module_api.commands);
        self.dictionary.insert(M::NAME, module_api.dictionary);
    }
}
