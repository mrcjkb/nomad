use smallvec::SmallVec;

use crate::action::ActionCtx;
use crate::backend::{Backend, BackendExt, BackendHandle, BackendMut};
use crate::command::{
    Command,
    CommandArg,
    CommandArgs,
    CommandCompletion,
    CompletionFn,
};
use crate::module::Module;
use crate::notify::{self, MaybeResult, ModulePath, Name};
use crate::plugin::Plugin;
use crate::util::OrderedMap;
use crate::{ByteOffset, NeovimCtx};

type CommandHandler<P, B> = Box<dyn FnMut(CommandArgs, &mut ActionCtx<P, B>)>;

type CommandCompletionFn =
    Box<dyn FnMut(CommandArgs, ByteOffset) -> Vec<CommandCompletion>>;

pub(crate) struct CommandBuilder<'a, P, B> {
    pub(crate) command_has_been_added: &'a mut bool,
    pub(crate) handlers: &'a mut CommandHandlers<P, B>,
    pub(crate) completions: &'a mut CommandCompletionFns,
}

pub(crate) struct CommandHandlers<P, B> {
    module_name: Name,
    inner: OrderedMap<Name, CommandHandler<P, B>>,
    submodules: OrderedMap<Name, Self>,
}

#[derive(Default)]
pub(crate) struct CommandCompletionFns {
    inner: OrderedMap<Name, CommandCompletionFn>,
    submodules: OrderedMap<Name, Self>,
}

struct MissingCommandError<'a, P, B>(&'a CommandHandlers<P, B>);

struct InvalidCommandError<'a, P, B>(
    &'a CommandHandlers<P, B>,
    CommandArg<'a>,
);

impl<P, B> CommandHandlers<P, B> {
    /// Pushes the list of valid commands and submodules to the given message.
    #[inline]
    fn push_valid(&self, message: &mut notify::Message) {
        let commands = self.inner.keys();
        let has_commands = commands.len() > 0;
        if has_commands {
            let valid_preface = if commands.len() == 1 {
                "the only valid command is "
            } else {
                "the valid commands are "
            };
            message
                .push_str(valid_preface)
                .push_comma_separated(commands, notify::SpanKind::Expected);
        }

        let submodules = self.submodules.keys();
        if submodules.len() > 0 {
            let valid_preface = if submodules.len() == 1 {
                "the only valid module is "
            } else {
                "the valid modules are "
            };
            message
                .push_str(if has_commands { "; " } else { "" })
                .push_str(valid_preface)
                .push_comma_separated(submodules, notify::SpanKind::Expected);
        }
    }
}

impl<'a, P: Plugin<B>, B: Backend> CommandBuilder<'a, P, B> {
    #[inline]
    pub(crate) fn new(
        command_has_been_added: &'a mut bool,
        handlers: &'a mut CommandHandlers<P, B>,
        completions: &'a mut CommandCompletionFns,
    ) -> Self {
        Self { command_has_been_added, handlers, completions }
    }

    #[track_caller]
    #[inline]
    pub(crate) fn add_command<Cmd>(&mut self, command: Cmd)
    where
        Cmd: Command<P, B>,
    {
        self.assert_namespace_is_available(Cmd::NAME);
        *self.command_has_been_added = true;
        self.completions.add_command(&command);
        self.handlers.add_command(command);
    }

    #[track_caller]
    #[inline]
    pub(crate) fn add_module<M>(&mut self) -> CommandBuilder<'_, P, B>
    where
        M: Module<P, B>,
    {
        self.assert_namespace_is_available(M::NAME);
        CommandBuilder {
            command_has_been_added: self.command_has_been_added,
            handlers: self.handlers.add_module::<M>(),
            completions: self.completions.add_module(M::NAME),
        }
    }

    #[track_caller]
    #[inline]
    fn assert_namespace_is_available(&self, namespace: &str) {
        let module_name = self.handlers.module_name;
        if self.handlers.inner.contains_key(namespace) {
            panic!(
                "a command with name {namespace:?} was already registered on \
                 {module_name:?}'s API",
            );
        }
        if self.completions.inner.contains_key(namespace) {
            panic!(
                "a submodule with name {namespace:?} was already registered \
                 on {module_name:?}'s API",
            );
        }
    }
}

impl<P: Plugin<B>, B: Backend> CommandHandlers<P, B> {
    #[inline]
    pub(crate) fn build(
        mut self,
        backend: BackendHandle<B>,
    ) -> impl FnMut(CommandArgs) + 'static {
        move |args: CommandArgs| {
            backend.with_mut(|backend| {
                let mut module_path = ModulePath::new(self.module_name);
                self.handle(args, &mut module_path, backend);
            })
        }
    }

    #[inline]
    pub(crate) fn new<M: Module<P, B>>() -> Self {
        Self {
            module_name: M::NAME,
            inner: Default::default(),
            submodules: Default::default(),
        }
    }

    #[inline]
    fn add_command<Cmd>(&mut self, mut command: Cmd)
    where
        Cmd: Command<P, B>,
    {
        let handler: CommandHandler<P, B> = Box::new(move |args, ctx| {
            let args = match Cmd::Args::try_from(args) {
                Ok(args) => args,
                Err(err) => {
                    ctx.emit_err(err);
                    return;
                },
            };
            if let Err(err) = command.call(args, ctx).into_result() {
                ctx.emit_err(err);
            }
        });
        self.inner.insert(Cmd::NAME, handler);
    }

    #[inline]
    fn add_module<M>(&mut self) -> &mut Self
    where
        M: Module<P, B>,
    {
        self.submodules.insert(M::NAME, Self::new::<M>())
    }

    #[inline]
    fn handle(
        &mut self,
        mut args: CommandArgs,
        module_path: &mut ModulePath,
        mut backend: BackendMut<B>,
    ) {
        let Some(arg) = args.pop_front() else {
            let err = MissingCommandError(self);
            let src = notify::Source { module_path, action_name: None };
            backend.emit_err::<P, _>(src, err);
            return;
        };

        if let Some((name, handler)) =
            self.inner.get_key_value_mut(arg.as_str())
        {
            let ctx = NeovimCtx::new(backend, module_path);
            (handler)(args, &mut ActionCtx::new(ctx, *name));
        } else if let Some(module) = self.submodules.get_mut(arg.as_str()) {
            module_path.push(module.module_name);
            module.handle(args, module_path, backend);
        } else {
            let err = InvalidCommandError(self, arg);
            let src = notify::Source { module_path, action_name: None };
            backend.emit_err::<P, _>(src, err);
        }
    }
}

impl CommandCompletionFns {
    #[inline]
    pub(crate) fn build(
        mut self,
    ) -> impl FnMut(CommandArgs, ByteOffset) -> Vec<CommandCompletion> + 'static
    {
        move |args: CommandArgs, cursor: ByteOffset| {
            self.complete(args, cursor)
        }
    }

    #[inline]
    fn add_command<Cmd, P, B>(&mut self, command: &Cmd)
    where
        Cmd: Command<P, B>,
        P: Plugin<B>,
        B: Backend,
    {
        let mut completion_fn = command.to_completion_fn();
        let completion_fn: CommandCompletionFn =
            Box::new(move |args, offset| {
                completion_fn.call(args, offset).into_iter().collect()
            });
        self.inner.insert(Cmd::NAME, completion_fn);
    }

    #[inline]
    fn add_module(&mut self, module_name: Name) -> &mut Self {
        self.submodules.insert(module_name, Default::default())
    }

    #[inline]
    fn complete(
        &mut self,
        mut args: CommandArgs,
        offset: ByteOffset,
    ) -> Vec<CommandCompletion> {
        debug_assert!(offset <= args.byte_len());

        let Some(arg) = args.pop_front() else {
            return self
                .inner
                .keys()
                .chain(self.submodules.keys())
                .copied()
                .map(CommandCompletion::from_static_str)
                .collect();
        };

        if offset <= arg.end() {
            let prefix = offset
                .checked_sub(arg.start())
                .map(|off| &arg.as_str()[..off.into()])
                .unwrap_or("");

            return self
                .inner
                .keys()
                .chain(self.submodules.keys())
                .filter(|&candidate| candidate.starts_with(prefix))
                .copied()
                .map(CommandCompletion::from_static_str)
                .collect();
        }

        let start_from = arg.end();
        let s = &args.as_str()[start_from.into()..];
        let args = CommandArgs::new(s);
        let offset = offset - start_from;

        if let Some(command) = self.inner.get_mut(arg.as_str()) {
            (command)(args, offset - start_from)
        } else if let Some(submodule) = self.submodules.get_mut(arg.as_str()) {
            submodule.complete(args, offset)
        } else {
            Vec::new()
        }
    }
}

impl<P, B> notify::Error<B> for MissingCommandError<'_, P, B>
where
    B: Backend,
{
    #[inline]
    fn to_message<P2>(
        &self,
        _: notify::Source,
    ) -> Option<(notify::Level, notify::Message)>
    where
        P2: Plugin<B>,
    {
        let Self(handlers) = self;
        let mut message = notify::Message::new();
        let missing = match (
            handlers.inner.is_empty(),
            handlers.submodules.is_empty(),
        ) {
            (false, false) => "command or submodule",
            (false, true) => "command",
            (true, false) => "submodule",
            (true, true) => unreachable!(),
        };
        message
            .push_str("missing ")
            .push_str(missing)
            .push_str(", ")
            .push_with(|message| handlers.push_valid(message));
        Some((notify::Level::Error, message))
    }
}

impl<P, B> notify::Error<B> for InvalidCommandError<'_, P, B>
where
    B: Backend,
{
    #[inline]
    fn to_message<P2>(
        &self,
        _: notify::Source,
    ) -> Option<(notify::Level, notify::Message)>
    where
        P2: Plugin<B>,
    {
        let Self(handlers, arg) = self;
        let mut message = notify::Message::new();
        let invalid = match (
            handlers.inner.is_empty(),
            handlers.submodules.is_empty(),
        ) {
            (false, false) => "argument",
            (false, true) => "command",
            (true, false) => "submodule",
            (true, true) => unreachable!(),
        };
        message
            .push_str("invalid ")
            .push_str(invalid)
            .push_str(" ")
            .push_invalid(arg.as_str())
            .push_str(", ");

        let levenshtein_threshold = 2;

        let mut guesses = handlers
            .inner
            .keys()
            .chain(handlers.submodules.keys())
            .map(|candidate| {
                let distance = strsim::levenshtein(candidate, arg);
                (candidate, distance)
            })
            .filter(|&(_, distance)| distance <= levenshtein_threshold)
            .collect::<SmallVec<[_; 2]>>();

        guesses.sort_by_key(|&(_, distance)| distance);

        if let Some((best_guess, _)) = guesses.first() {
            message
                .push_str("did you mean ")
                .push_expected(best_guess)
                .push_str("?");
        } else {
            handlers.push_valid(&mut message);
        }

        Some((notify::Level::Error, message))
    }
}
