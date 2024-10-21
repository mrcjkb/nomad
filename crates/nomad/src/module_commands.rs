use fxhash::FxHashMap;

use crate::action::ActionName;
use crate::diagnostics::{
    DiagnosticMessage,
    DiagnosticSource,
    HighlightGroup,
    Level,
};
use crate::{Action, CommandArgs, Module, ModuleName};

pub(super) struct ModuleCommands {
    /// The name of the module these commands belong to.
    pub(super) module_name: ModuleName,

    /// The command to run when no command is specified.
    pub(super) default_command: Option<Command>,

    /// Map from command name to the corresponding [`Command`].
    pub(super) commands: FxHashMap<ActionName, Command>,
}

impl ModuleCommands {
    #[track_caller]
    pub(crate) fn add_command<T>(&mut self, command: T)
    where
        T: Action<Return = ()>,
        T::Args: Clone
            + for<'a> TryFrom<
                &'a mut CommandArgs,
                Error: Into<DiagnosticMessage>,
            >,
    {
        if self.module_name != T::Module::NAME {
            panic!(
                "trying to register a command for module '{}' in the API for \
                 module '{}'",
                T::Module::NAME,
                self.module_name
            );
        }
        if self.commands.contains_key(&T::NAME) {
            panic!(
                "a command with the name '{}' already exists in the API for \
                 module '{}'",
                T::NAME,
                self.module_name
            );
        }
        self.commands.insert(T::NAME, Command::new(command));
    }

    #[track_caller]
    fn add_default_command<T>(&mut self, command: T)
    where
        T: Action,
        T::Args: Clone
            + for<'a> TryFrom<
                &'a mut CommandArgs,
                Error: Into<DiagnosticMessage>,
            >,
    {
        if self.module_name != T::Module::NAME {
            panic!(
                "trying to register a command for module '{}' in the API for \
                 module '{}'",
                T::Module::NAME,
                self.module_name
            );
        }

        if self.default_command.is_some() {
            panic!(
                "a default command has already been set for module '{}'",
                self.module_name
            );
        }

        self.default_command = Some(Command::new(command));
    }

    pub(crate) fn default_command(&self) -> Option<&Command> {
        self.default_command.as_ref()
    }

    pub(crate) fn command(
        &self,
        command_name: ActionName,
    ) -> Option<&Command> {
        self.commands.get(&command_name)
    }

    pub(crate) fn new<M: Module>() -> Self {
        Self {
            module_name: M::NAME,
            default_command: None,
            commands: FxHashMap::default(),
        }
    }
}

struct Command {
    inner: Box<dyn Fn(CommandArgs)>,
}

impl Command {
    fn new<T>(command: T) -> Self
    where
        T: Action<Return = ()>,
        T::Args: Clone
            + for<'a> TryFrom<
                &'a mut CommandArgs,
                Error: Into<DiagnosticMessage>,
            >,
    {
        Self {
            inner: Box::new(move |mut args| {
                let args = match T::Args::try_from(&mut args) {
                    Ok(args) => args,
                    Err(err) => {
                        let message = err.into();
                        let source = DiagnosticSource {
                            name: T::Module::NAME,
                            highlight_group: HighlightGroup::Error,
                            level: Level::Error,
                        };
                        let diagnostic = message.into_diagnostic(source);
                        args.emit_diagnostic(diagnostic);
                        return;
                    },
                };
                command.execute(args);
            }),
        }
    }
}
