use core::convert::Infallible;
use std::collections::hash_map::Values;
use std::collections::HashMap;

use nvim::api::{self, opts, types};
use nvim::Function;

use crate::prelude::*;
use crate::warning::ChunkExt;

/// TODO: docs
#[derive(Default)]
pub(crate) struct Command {
    map: HashMap<ModuleId, ModuleCommands>,
}

impl Command {
    const NAME: &'static str = "Nomad";

    #[inline]
    pub(crate) fn add_module<M: Module>(&mut self, commands: ModuleCommands) {
        self.map.insert(M::NAME.id(), commands);
    }

    #[inline]
    pub(crate) fn create(self, ctx: Ctx) {
        let opts = opts::CreateCommandOpts::builder()
            .nargs(types::CommandNArgs::OneOrMore)
            .build();

        api::create_user_command(Self::NAME, self.into_func(ctx), &opts)
            .expect("all the arguments are valid");
    }

    #[inline]
    fn into_func(self, ctx: Ctx) -> Function<types::CommandArgs, ()> {
        let Self { map } = self;

        Function::from_fn(move |args: types::CommandArgs| {
            let mut args = CommandArgs::from(args);

            let Some(first) = args.split_first() else {
                unreachable!("Nomad needs OneOrMore arguments")
            };

            let Some(commands) = map.get(&ModuleId::from_module_name(first))
            else {
                Warning::new().msg(UnknownModule(first).into()).print();
                return Ok(());
            };

            let Some(action_name) = args.split_first() else {
                Warning::new()
                    .module(commands.module_name)
                    .msg(MissingAction.into())
                    .print();

                return Ok(());
            };

            match commands.get(action_name) {
                Ok(command) => ctx.with_set(|ctx| command.execute(args, ctx)),

                Err(err) => Warning::new()
                    .module(commands.module_name)
                    .msg(err.into())
                    .print(),
            }

            Ok::<_, Infallible>(())
        })
    }
}

struct UnknownModule<'a>(&'a str);

impl From<UnknownModule<'_>> for WarningMsg {
    #[inline]
    fn from(UnknownModule(name): UnknownModule) -> WarningMsg {
        let mut msg = WarningMsg::new();
        msg.add("unknown module ");
        msg.add(name.highlight());
        msg
    }
}

struct MissingAction;

impl From<MissingAction> for WarningMsg {
    #[inline]
    fn from(_: MissingAction) -> WarningMsg {
        let mut msg = WarningMsg::new();
        msg.add("no action provided");
        msg
    }
}

/// TODO: docs
pub(crate) struct ModuleCommands {
    map: HashMap<ActionId, ModuleCommand>,
    module_name: ModuleName,
}

impl ModuleCommands {
    #[inline]
    pub(crate) fn add<M, A>(&mut self, action: A)
    where
        M: Module,
        A: Action<M, Return = ()>,
        A::Args: TryFrom<CommandArgs>,
        <A::Args as TryFrom<CommandArgs>>::Error: Into<WarningMsg>,
    {
        self.map.insert(A::NAME.id(), ModuleCommand::new(action));
    }

    #[inline]
    fn get<'this, 'a>(
        &'this self,
        action: &'a str,
    ) -> Result<&'this ModuleCommand, UnknownAction<'a, 'this>> {
        self.map.get(&ActionId::from_action_name(action)).ok_or_else(|| {
            UnknownAction { action, valid_actions: self.map.values() }
        })
    }

    #[inline]
    pub(crate) fn new(module_name: ModuleName) -> Self {
        Self { map: HashMap::new(), module_name }
    }
}

struct UnknownAction<'action, 'values> {
    action: &'action str,
    valid_actions: Values<'values, ActionId, ModuleCommand>,
}

impl From<UnknownAction<'_, '_>> for WarningMsg {
    #[inline]
    fn from(
        UnknownAction { action, mut valid_actions }: UnknownAction,
    ) -> WarningMsg {
        let mut msg = WarningMsg::new();

        msg.add("invalid action ").add(action.highlight());

        let num_valid = valid_actions.len();

        match num_valid {
            0 => {},

            1 => {
                msg.add(", the only valid action is ").add(
                    valid_actions
                        .next()
                        .unwrap()
                        .action_name
                        .as_str()
                        .highlight(),
                );
            },

            _ => {
                msg.add(", the valid actions are ");

                for (idx, action) in valid_actions.enumerate() {
                    msg.add(action.action_name.as_str().highlight());

                    let is_last = idx + 1 == num_valid;

                    if is_last {
                        break;
                    }

                    let is_second_to_last = idx + 2 == num_valid;

                    if is_second_to_last {
                        msg.add(" and ");
                    } else {
                        msg.add(", ");
                    }
                }
            },
        }

        msg
    }
}

struct ModuleCommand {
    #[allow(clippy::type_complexity)]
    action: Box<dyn Fn(CommandArgs, &mut SetCtx) -> Result<(), WarningMsg>>,
    action_name: ActionName,
    module_name: ModuleName,
}

impl ModuleCommand {
    #[inline]
    fn execute(&self, args: CommandArgs, ctx: &mut SetCtx) {
        if let Err(warning_msg) = (self.action)(args, ctx) {
            Warning::new()
                .module(self.module_name)
                .action(self.action_name)
                .msg(warning_msg)
                .print();
        }
    }

    #[inline]
    fn new<M, A>(action: A) -> Self
    where
        M: Module,
        A: Action<M, Return = ()>,
        A::Args: TryFrom<CommandArgs>,
        <A::Args as TryFrom<CommandArgs>>::Error: Into<WarningMsg>,
    {
        let action = move |args, ctx: &mut _| {
            A::Args::try_from(args).map_err(Into::into).and_then(|args| {
                action.execute(args, ctx).into_result().map_err(Into::into)
            })
        };

        Self {
            action: Box::new(action),
            action_name: A::NAME,
            module_name: M::NAME,
        }
    }
}

/// TODO: docs
pub struct CommandArgs {
    /// TODO: docs
    args: Vec<String>,

    /// TODO: docs
    consumed: usize,
}

impl From<types::CommandArgs> for CommandArgs {
    #[inline]
    fn from(args: types::CommandArgs) -> Self {
        Self::new(args.fargs)
    }
}

impl CommandArgs {
    /// TODO: docs
    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = String> {
        self.args.into_iter().skip(self.consumed)
    }

    /// TODO: docs
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// TODO: docs
    #[inline]
    pub fn len(&self) -> usize {
        self.args.len() - self.consumed
    }

    /// TODO: docs
    #[inline]
    fn new(args: Vec<String>) -> Self {
        Self { args, consumed: 0 }
    }

    /// TODO: docs
    #[inline]
    fn split_first(&mut self) -> Option<&str> {
        self.args
            .get(self.consumed)
            .map(String::as_str)
            .inspect(|_| self.consumed += 1)
    }
}

impl TryFrom<CommandArgs> for () {
    type Error = CommandArgsNotEmtpy;

    #[inline]
    fn try_from(args: CommandArgs) -> Result<Self, Self::Error> {
        if args.is_empty() {
            Ok(())
        } else {
            Err(CommandArgsNotEmtpy(args))
        }
    }
}

/// An error indicating a command's arguments were not empty.
pub struct CommandArgsNotEmtpy(CommandArgs);

impl From<CommandArgsNotEmtpy> for WarningMsg {
    #[inline]
    fn from(CommandArgsNotEmtpy(args): CommandArgsNotEmtpy) -> WarningMsg {
        assert!(!args.is_empty());

        let mut msg = WarningMsg::new();

        msg.add("expected no arguments, but got ");

        let num_args = args.len();

        for (idx, arg) in args.into_iter().enumerate() {
            msg.add(arg.highlight());

            let is_last = idx + 1 == num_args;

            if is_last {
                break;
            }

            let is_second_to_last = idx + 2 == num_args;

            if is_second_to_last {
                msg.add(" and ");
            } else {
                msg.add(", ");
            }
        }

        msg
    }
}
