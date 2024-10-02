use core::ops::AddAssign;
use std::collections::HashMap;

use nvim_oxi::lua::ffi::State as LuaState;
use nvim_oxi::{
    api,
    lua,
    Dictionary as NvimDictionary,
    Function as NvimFunction,
    Object as NvimObject,
};

use super::module_api::ModuleCommands;
use super::{ModuleApi, Neovim};
use crate::Nomad;

/// TODO: docs.
const MAD_CMD_NAME: &str = "Mad";

/// TODO: docs.
const SETUP_FN_NAME: &str = "setup";

/// TODO: docs.
#[derive(Default)]
pub struct Api {
    commands: Commands,
    dict: NvimDictionary,
}

impl AddAssign<ModuleApi> for Api {
    #[track_caller]
    fn add_assign(&mut self, module_api: ModuleApi) {
        if self.dict.get(&module_api.name).is_some() {
            panic!(
                "a module with the name '{}' has already been added to the \
                 API",
                module_api.name
            );
        }

        if module_api.name == SETUP_FN_NAME {
            panic!(
                "got a module with the name '{}', which is reserved for the \
                 setup function",
                module_api.name
            );
        }

        self.dict.insert(module_api.name, module_api.inner);
    }
}

fn setup(_obj: NvimObject) {}

#[derive(Default)]
struct Commands {
    /// Map from module name to the commands for that module.
    map: HashMap<&'static str, ModuleCommands>,
}

impl Commands {
    fn create_mad_command(self) {
        let opts = api::opts::CreateCommandOpts::builder()
            .nargs(api::types::CommandNArgs::Any)
            .build();

        api::create_user_command(
            MAD_CMD_NAME,
            nvim_oxi::Function::from_fn(self.on_execute()),
            &opts,
        )
        .expect("all the arguments are valid");
    }

    fn on_execute(self) -> Box<dyn Fn(api::types::CommandArgs) + 'static> {
        todo!();
    }
}

impl AddAssign<ModuleCommands> for Commands {
    #[track_caller]
    fn add_assign(&mut self, commands: ModuleCommands) {
        if self.map.contains_key(&commands.module_name) {
            panic!(
                "a module with the name '{}' has already been added to the \
                 API",
                commands.module_name
            );
        }

        self.map.insert(commands.module_name, commands);
    }
}

impl lua::Pushable for Api {
    unsafe fn push(mut self, state: *mut LuaState) -> Result<i32, lua::Error> {
        self.commands.create_mad_command();

        let setup = NvimFunction::from_fn(|obj| setup(obj));
        self.dict.insert(SETUP_FN_NAME, setup);
        self.dict.push(state)
    }
}

impl lua::Pushable for Nomad<Neovim> {
    unsafe fn push(mut self, state: *mut LuaState) -> Result<i32, lua::Error> {
        self.start_modules();
        self.into_api().push(state)
    }
}
