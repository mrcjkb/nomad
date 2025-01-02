//! TODO: docs.

use serde::de::DeserializeOwned;

use crate::api::{Api, ModuleApi};
use crate::command::Command;
use crate::{
    Backend,
    BackendExt,
    BackendHandle,
    Function,
    MaybeResult,
    NeovimCtx,
    Plugin,
    notify,
};

/// TODO: docs.
pub trait Module<B: Backend>: 'static + Sized {
    /// TODO: docs.
    const NAME: &'static ModuleName;

    /// TODO: docs.
    type Config: DeserializeOwned;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api<P: Plugin<B>>(&self, ctx: ApiCtx<'_, '_, Self, P, B>);

    /// TODO: docs.
    fn on_config_changed(
        &mut self,
        new_config: Self::Config,
        ctx: NeovimCtx<'_, B>,
    );

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

/// TODO: docs.
pub struct ApiCtx<'a, 'b, M: Module<B>, P: Plugin<B>, B: Backend> {
    module_api: &'a mut <B::Api<P> as Api<P, B>>::ModuleApi<'b, M>,
    command_builder: CommandBuilder<'a, B>,
    namespace: &'a mut notify::Namespace,
    backend: &'b BackendHandle<B>,
}

/// TODO: docs.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ModuleName(str);

impl<'a, 'b, M, P, B> ApiCtx<'a, 'b, M, P, B>
where
    M: Module<B>,
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn with_command<Cmd>(mut self, command: Cmd) -> Self
    where
        Cmd: Command<B>,
    {
        self.command_builder.add_command(command);
        self
    }

    /// TODO: docs.
    #[track_caller]
    #[inline]
    pub fn with_function<Fun>(self, mut function: Fun) -> Self
    where
        Fun: Function<B>,
    {
        let backend = self.backend.clone();
        let mut namespace = self.namespace.clone();
        namespace.set_action(Fun::NAME);
        let fun = move |value| {
            let fun = &mut function;
            let namespace = &namespace;
            backend.with_mut(move |mut backend| {
                let args = backend.deserialize::<Fun::Args>(value).map_err(
                    |err| {
                        backend.emit_err(namespace, &err);
                        FunctionError::Deserialize(err)
                    },
                )?;

                let ret = fun
                    .call(args, NeovimCtx::new(backend.as_mut()))
                    .into_result()
                    .map_err(|err| {
                        // Even though the error is bound to 'static, Rust
                        // thinks that the error captures some lifetime due to
                        // `Function::call()` returning an `impl MaybeResult`.
                        //
                        // Should be the same problem as
                        // https://github.com/rust-lang/rust/issues/42940
                        //
                        // FIXME: Is there a better way around this than boxing
                        // the error?
                        Box::new(err) as Box<dyn notify::Error>
                    })
                    .map_err(|err| {
                        backend.emit_err(namespace, &err);
                        FunctionError::Call(err)
                    })?;

                backend.serialize(&ret).map_err(|err| {
                    backend.emit_err(namespace, &err);
                    FunctionError::Serialize(err)
                })
            })
        };
        self.module_api.add_function(Fun::NAME, fun);
        self
    }

    /// TODO: docs.
    #[inline]
    pub fn with_module<Mod>(mut self, module: Mod) -> Self
    where
        Mod: Module<B>,
    {
        let mut module_api = self.module_api.as_module::<Mod>();
        let command_builder = self.command_builder.add_module::<Mod>();
        self.namespace.push_module(Mod::NAME);
        let api_ctx = ApiCtx::new(
            &mut module_api,
            command_builder,
            self.namespace,
            self.backend,
        );
        Module::api(&module, api_ctx);
        module_api.finish();
        self.namespace.pop();
        self
    }

    #[inline]
    pub(crate) fn new(
        module_api: &'a mut <B::Api<P> as Api<P, B>>::ModuleApi<'b, M>,
        command_builder: CommandBuilder<'a, B>,
        namespace: &'a mut notify::Namespace,
        backend: &'b BackendHandle<B>,
    ) -> Self {
        Self { module_api, command_builder, namespace, backend }
    }
}

impl ModuleName {
    /// TODO: docs.
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /// TODO: docs.
    #[inline]
    pub const fn new(name: &str) -> &Self {
        assert!(!name.is_empty());
        assert!(name.len() <= 24);
        // SAFETY: `ModuleName` is a `repr(transparent)` newtype around `str`.
        unsafe { &*(name as *const str as *const Self) }
    }

    /// TODO: docs.
    #[inline]
    pub const fn uppercase_first(&self) -> &Self {
        todo!();
    }
}

enum FunctionError<D, C, S> {
    Deserialize(D),
    Call(C),
    Serialize(S),
}

impl<D, C, S> notify::Error for FunctionError<D, C, S>
where
    D: notify::Error,
    C: notify::Error,
    S: notify::Error,
{
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        match self {
            Self::Deserialize(err) => err.to_level(),
            Self::Call(err) => err.to_level(),
            Self::Serialize(err) => err.to_level(),
        }
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        match self {
            Self::Deserialize(err) => err.to_message(),
            Self::Call(err) => err.to_message(),
            Self::Serialize(err) => err.to_message(),
        }
    }
}

pub(crate) use command_builder::{
    CommandBuilder,
    CommandCompletionFns,
    CommandHandlers,
};

mod command_builder {
    use core::borrow::Borrow;

    use super::{Module, ModuleName};
    use crate::backend::BackendExt;
    use crate::backend_handle::BackendHandle;
    use crate::command::{
        Command,
        CommandArgs,
        CommandCompletion,
        CompletionFn,
    };
    use crate::{Backend, ByteOffset, MaybeResult, NeovimCtx, notify};

    type CommandHandler<B> =
        Box<dyn FnMut(CommandArgs, &mut notify::Namespace, NeovimCtx<B>)>;

    type CommandCompletionFn =
        Box<dyn FnMut(CommandArgs, ByteOffset) -> Vec<CommandCompletion>>;

    pub(crate) struct CommandBuilder<'a, B> {
        pub(crate) handlers: &'a mut CommandHandlers<B>,
        pub(crate) completions: &'a mut CommandCompletionFns,
    }

    pub(crate) struct CommandHandlers<B> {
        module_name: &'static ModuleName,
        inner: OrderedMap<&'static str, CommandHandler<B>>,
        submodules: OrderedMap<&'static str, Self>,
    }

    #[derive(Default)]
    pub(crate) struct CommandCompletionFns {
        inner: OrderedMap<&'static str, CommandCompletionFn>,
        submodules: OrderedMap<&'static str, Self>,
    }

    struct OrderedMap<K, V> {
        inner: Vec<(K, V)>,
    }

    struct MissingCommandError<'a, B>(&'a CommandHandlers<B>);

    struct UnknownCommandError<'a, B>(&'a CommandHandlers<B>, &'a str);

    impl<'a, B: Backend> CommandBuilder<'a, B> {
        #[inline]
        pub(crate) fn new(
            handlers: &'a mut CommandHandlers<B>,
            completions: &'a mut CommandCompletionFns,
        ) -> Self {
            Self { handlers, completions }
        }

        #[track_caller]
        #[inline]
        pub(super) fn add_command<Cmd>(&mut self, command: Cmd)
        where
            Cmd: Command<B>,
        {
            self.assert_namespace_is_available(Cmd::NAME.as_str());
            self.completions.add_command(&command);
            self.handlers.add_command(command);
        }

        #[track_caller]
        #[inline]
        pub(super) fn add_module<M>(&mut self) -> CommandBuilder<'_, B>
        where
            M: Module<B>,
        {
            self.assert_namespace_is_available(M::NAME.as_str());
            CommandBuilder {
                handlers: self.handlers.add_module::<M>(),
                completions: self.completions.add_module(M::NAME),
            }
        }

        #[track_caller]
        #[inline]
        fn assert_namespace_is_available(&self, namespace: &str) {
            let module_name = self.handlers.module_name.as_str();
            if self.handlers.inner.contains_key(&namespace) {
                panic!(
                    "a command with name {namespace:?} was already \
                     registered on {module_name:?}'s API",
                );
            }
            if self.completions.inner.contains_key(&namespace) {
                panic!(
                    "a submodule with name {namespace:?} was already \
                     registered on {module_name:?}'s API",
                );
            }
        }
    }

    impl<B: Backend> CommandHandlers<B> {
        #[inline]
        pub(crate) fn build(
            mut self,
            backend: BackendHandle<B>,
        ) -> impl FnMut(CommandArgs) + 'static {
            move |args: CommandArgs| {
                backend.with_mut(|backend| {
                    let mut namespace = notify::Namespace::default();
                    self.handle(args, &mut namespace, NeovimCtx::new(backend));
                })
            }
        }

        #[inline]
        pub(crate) fn new<M: Module<B>>() -> Self {
            Self {
                module_name: M::NAME,
                inner: Default::default(),
                submodules: Default::default(),
            }
        }

        #[inline]
        fn add_command<Cmd>(&mut self, mut command: Cmd)
        where
            Cmd: Command<B>,
        {
            let handler: CommandHandler<B> =
                Box::new(move |args, namespace, mut ctx| {
                    namespace.set_action(Cmd::NAME);
                    let args = match Cmd::Args::try_from(args) {
                        Ok(args) => args,
                        Err(err) => {
                            ctx.backend_mut().emit_err(namespace, &err);
                            return;
                        },
                    };
                    if let Err(err) =
                        command.call(args, ctx.as_mut()).into_result()
                    {
                        ctx.backend_mut().emit_err(namespace, &err);
                    }
                });
            self.inner.insert(Cmd::NAME.as_str(), handler);
        }

        #[inline]
        fn add_module<M: Module<B>>(&mut self) -> &mut Self {
            self.submodules.insert(M::NAME.as_str(), Self::new::<M>())
        }

        #[inline]
        fn handle(
            &mut self,
            mut args: CommandArgs,
            namespace: &mut notify::Namespace,
            mut ctx: NeovimCtx<B>,
        ) {
            namespace.push_module(self.module_name);

            let Some(arg) = args.next() else {
                let err = MissingCommandError(self);
                return ctx.backend_mut().emit_err(namespace, &err);
            };

            if let Some(handler) = self.inner.get_mut(arg) {
                (handler)(args, namespace, ctx);
            } else if let Some(module) = self.submodules.get_mut(arg) {
                module.handle(args, namespace, ctx);
            } else {
                let err = UnknownCommandError(self, arg);
                ctx.backend_mut().emit_err(namespace, &err);
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
        fn add_command<Cmd, B>(&mut self, command: &Cmd)
        where
            Cmd: Command<B>,
            B: Backend,
        {
            let mut completion_fn = command.to_completion_fn();
            let completion_fn: CommandCompletionFn =
                Box::new(move |args, offset| {
                    completion_fn.call(args, offset).into_iter().collect()
                });
            self.inner.insert(Cmd::NAME.as_str(), completion_fn);
        }

        #[inline]
        fn add_module(
            &mut self,
            module_name: &'static ModuleName,
        ) -> &mut Self {
            self.submodules.insert(module_name.as_str(), Default::default())
        }

        #[inline]
        fn complete(
            &mut self,
            args: CommandArgs,
            offset: ByteOffset,
        ) -> Vec<CommandCompletion> {
            debug_assert!(offset <= args.byte_len());
            todo!();
        }
    }

    impl<K: Ord, V> OrderedMap<K, V> {
        #[inline]
        fn contains_key(&self, key: K) -> bool {
            self.get_idx(&key).is_ok()
        }

        #[inline]
        fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            let idx = self.get_idx(key).ok()?;
            Some(&self.inner[idx].1)
        }

        #[inline]
        fn get_idx<Q>(&self, key: &Q) -> Result<usize, usize>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            self.inner.binary_search_by(|(probe, _)| {
                Borrow::<Q>::borrow(probe).cmp(key)
            })
        }

        #[inline]
        fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            let idx = self.get_idx(key).ok()?;
            Some(&mut self.inner[idx].1)
        }

        #[inline]
        fn insert(&mut self, key: K, value: V) -> &mut V {
            let idx = self.get_idx(&key).unwrap_or_else(|x| x);
            self.inner.insert(idx, (key, value));
            &mut self.inner[idx].1
        }
    }

    impl<K, V> Default for OrderedMap<K, V> {
        #[inline]
        fn default() -> Self {
            Self { inner: Vec::new() }
        }
    }

    impl<B> notify::Error for MissingCommandError<'_, B> {
        #[inline]
        fn to_level(&self) -> Option<notify::Level> {
            Some(notify::Level::Error)
        }

        #[inline]
        fn to_message(&self) -> notify::Message {
            todo!()
        }
    }

    impl<B> notify::Error for UnknownCommandError<'_, B> {
        #[inline]
        fn to_level(&self) -> Option<notify::Level> {
            Some(notify::Level::Error)
        }

        #[inline]
        fn to_message(&self) -> notify::Message {
            todo!()
        }
    }
}
