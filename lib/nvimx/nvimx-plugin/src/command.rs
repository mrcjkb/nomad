use fxhash::FxHashMap;
use nvimx_common::oxi::{self, api};
use nvimx_diagnostics::{
    DiagnosticMessage,
    DiagnosticSource,
    HighlightGroup,
    Level,
};

use crate::action_name::ActionNameStr;
use crate::module_name::{ModuleName, ModuleNameStr};
use crate::module_subcommands::ModuleSubCommands;
use crate::plugin::Plugin;
use crate::subcommand_args::{SubCommandArg, SubCommandArgs};

pub(crate) struct Command {
    command_name: &'static str,
    /// A map from module name to the subcommands for that module.
    subcommands: FxHashMap<ModuleNameStr, ModuleSubCommands>,
}

impl Command {
    pub(crate) fn add_module(&mut self, module_commands: ModuleSubCommands) {
        let module_name = module_commands.module_name.as_str();
        if self.subcommands.contains_key(&module_name) {
            panic!(
                "subcommands from a module named '{}' have already been added",
                module_name
            );
        }
        self.subcommands.insert(module_name, module_commands);
    }

    pub(crate) fn create(mut self) {
        let opts = api::opts::CreateCommandOpts::builder()
            .nargs(api::types::CommandNArgs::Any)
            .build();

        api::create_user_command(
            self.command_name,
            move |args: api::types::CommandArgs| {
                let args =
                    SubCommandArgs::new(args.args.as_deref().unwrap_or(""));
                if let Err(err) = self.call(args) {
                    err.emit()
                }
            },
            &opts,
        )
        .expect("all the arguments are valid");
    }

    pub(crate) fn new<P: Plugin>() -> Self {
        Self {
            command_name: P::COMMAND_NAME,
            subcommands: FxHashMap::default(),
        }
    }

    fn call<'a>(
        &mut self,
        mut args: SubCommandArgs<'a>,
    ) -> Result<(), CommandError<'a>> {
        let Some(module_name) = args.pop_front() else {
            return Err(CommandError::MissingModule {
                valid: self.subcommands.keys().copied().collect(),
            });
        };

        let Some(module_subcommands) = self.subcommands.get_mut(&*module_name)
        else {
            return Err(CommandError::UnknownModule {
                module_name,
                valid: self.subcommands.keys().copied().collect(),
            });
        };

        let Some(subcommand_name) = args.pop_front() else {
            return if let Some(default) =
                module_subcommands.default_subcommand()
            {
                default.call(args);
                Ok(())
            } else {
                Err(CommandError::MissingSubCommand {
                    module_name: module_subcommands.module_name,
                    valid: module_subcommands.subcommand_names().collect(),
                })
            };
        };

        match module_subcommands.subcommand(&subcommand_name) {
            Some(subcommand) => {
                subcommand.call(args);
                Ok(())
            },
            None => Err(CommandError::UnknownSubCommand {
                module_name: module_subcommands.module_name,
                subcommand_name,
                valid: module_subcommands.subcommand_names().collect(),
            }),
        }
    }
}

/// The type of error that can occur when [`call`](NomadCommand::call)ing the
/// [`NomadCommand`].
enum CommandError<'args> {
    MissingSubCommand {
        module_name: ModuleName,
        valid: Vec<ActionNameStr>,
    },
    MissingModule {
        valid: Vec<ModuleNameStr>,
    },
    UnknownSubCommand {
        module_name: ModuleName,
        subcommand_name: SubCommandArg<'args>,
        valid: Vec<ActionNameStr>,
    },
    UnknownModule {
        module_name: SubCommandArg<'args>,
        valid: Vec<ModuleNameStr>,
    },
}

impl CommandError<'_> {
    fn emit(self) {
        self.message().emit(Level::Warning, self.source());
    }

    fn message(&self) -> DiagnosticMessage {
        let mut message = DiagnosticMessage::new();
        match self {
            Self::MissingSubCommand { valid, .. } => {
                message
                    .push_str(
                        "missing subcommand, the valid subcommands are: ",
                    )
                    .push_comma_separated(valid, HighlightGroup::special());
            },
            Self::MissingModule { valid } => {
                message
                    .push_str("missing module, the valid modules are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },

            Self::UnknownSubCommand { subcommand_name, valid, .. } => {
                message
                    .push_str("unknown subcommand '")
                    .push_str_highlighted(
                        subcommand_name,
                        HighlightGroup::warning(),
                    )
                    .push_str("', the valid subcommands are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },
            Self::UnknownModule { module_name, valid } => {
                message
                    .push_str("unknown module '")
                    .push_str_highlighted(
                        module_name,
                        HighlightGroup::warning(),
                    )
                    .push_str("', the valid modules are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },
        }
        message
    }

    fn source(&self) -> DiagnosticSource {
        let mut source = DiagnosticSource::new();
        match self {
            Self::UnknownSubCommand { module_name, .. }
            | Self::MissingSubCommand { module_name, .. } => {
                source.push_segment(module_name.as_str());
            },
            _ => (),
        }
        source
    }
}
