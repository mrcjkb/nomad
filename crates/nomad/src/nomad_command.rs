use fxhash::FxHashMap;
use nvim_oxi::api;

use crate::action::ActionNameStr;
use crate::command_args::CommandArgs;
use crate::module_commands::ModuleCommands;
use crate::module_name::ModuleNameStr;

#[derive(Default)]
pub(crate) struct NomadCommand {
    /// A map from module name to the commands for that module.
    commands: FxHashMap<ModuleNameStr, ModuleCommands>,
}

impl NomadCommand {
    const NAME: &'static str = "Mad";

    #[track_caller]
    pub(crate) fn add_module(&mut self, module_commands: ModuleCommands) {
        let module_name = module_commands.module_name.as_str();
        if self.commands.contains_key(&module_name) {
            panic!(
                "commands from a module named '{}' have already been added",
                module_name
            );
        }
        self.commands.insert(module_name, module_commands);
    }

    pub(crate) fn create(self) {
        let opts = api::opts::CreateCommandOpts::builder()
            .nargs(api::types::CommandNArgs::Any)
            .build();

        api::create_user_command(
            Self::NAME,
            move |args| {
                let args = CommandArgs::from(args);
                if let Err(err) = self.call(args) {
                    err.emit()
                }
            },
            &opts,
        )
        .expect("all the arguments are valid");
    }

    fn call(&self, mut args: CommandArgs) -> Result<(), NomadCommandError> {
        let Some(module_name) = args.pop_front() else {
            return Err(NomadCommandError::MissingModule {
                valid: self.commands.keys().copied().collect(),
            });
        };

        let Some(module_commands) = self.commands.get(&module_name.as_str())
        else {
            return Err(NomadCommandError::UnknownModule {
                module_name,
                valid: self.commands.keys().copied().collect(),
            });
        };

        let Some(command_name) = args.pop_front() else {
            return if let Some(default) = module_commands.default_command() {
                default(args);
                Ok(())
            } else {
                Err(NomadCommandError::MissingCommand {
                    valid: module_commands.command_names().collect(),
                })
            };
        };

        match module_commands.command(&command_name.as_str()) {
            Some(command) => {
                command(args);
                Ok(())
            },
            None => Err(NomadCommandError::UnknownCommand {
                command_name,
                valid: module_commands.command_names().collect(),
            }),
        }
    }
}

/// The type of error that can occur when [`call`](NomadCommand::call)ing the
/// [`NomadCommand`].
enum NomadCommandError {
    /// TODO: docs.
    MissingCommand { valid: Vec<ActionNameStr> },

    /// TODO: docs.
    MissingModule { valid: Vec<ModuleNameStr> },

    /// TODO: docs.
    UnknownCommand { command_name: String, valid: Vec<ActionNameStr> },

    /// TODO: docs.
    UnknownModule { module_name: String, valid: Vec<ModuleNameStr> },
}

impl NomadCommandError {
    fn emit(self) {}
}
