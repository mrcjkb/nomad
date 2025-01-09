use crate::NeovimCtx;
use crate::action::ActionCtx;
use crate::backend::{
    Api,
    Backend,
    BackendExt,
    BackendHandle,
    BackendMut,
    Key,
    MapAccess,
    ModuleApi,
    Value,
};
use crate::command::{Command, CommandBuilder};
use crate::module::{Constant, Function, Module};
use crate::notify::{self, Error, MaybeResult, ModulePath, Name};
use crate::plugin::Plugin;
use crate::util::OrderedMap;

/// TODO: docs.
pub struct ApiCtx<'a, 'b, M: Module<P, B>, P: Plugin<B>, B: Backend> {
    module_api: &'a mut <B::Api<P> as Api<P, B>>::ModuleApi<'b, M>,
    command_builder: CommandBuilder<'a, P, B>,
    config_builder: &'a mut ConfigFnBuilder<P, B>,
    module_path: &'a mut ModulePath,
    backend: &'a BackendHandle<B>,
}

pub(crate) struct ConfigFnBuilder<P: Plugin<B>, B: Backend> {
    module_name: Name,
    config_handler: Box<dyn FnMut(B::ApiValue, &mut NeovimCtx<P, B>)>,
    submodules: OrderedMap<Name, Self>,
}

impl<'a, 'b, M, P, B> ApiCtx<'a, 'b, M, P, B>
where
    M: Module<P, B>,
    P: Plugin<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn with_command<Cmd>(&mut self, command: Cmd) -> &mut Self
    where
        Cmd: Command<P, B>,
    {
        self.command_builder.add_command(command);
        self
    }

    /// TODO: docs.
    #[track_caller]
    #[inline]
    pub fn with_constant<Const>(&mut self, value: Const) -> &mut Self
    where
        Const: Constant,
    {
        let value = self.backend.with_mut(|mut backend| {
            match backend.serialize(&value).into_result() {
                Ok(value) => value,
                Err(err) => {
                    let source = notify::Source {
                        module_path: self.module_path,
                        action_name: Some(Const::NAME),
                    };
                    let msg = err.to_message::<P>(source).map(|(_, msg)| msg);
                    panic!(
                        "couldn't serialize {:?}{colon}{reason:?}",
                        Const::NAME,
                        colon = if msg.is_some() { ": " } else { "" },
                        reason =
                            msg.as_ref().map(|msg| msg.as_str()).unwrap_or(""),
                    );
                },
            }
        });
        self.module_api.add_constant(Const::NAME, value);
        self
    }

    /// TODO: docs.
    #[track_caller]
    #[inline]
    pub fn with_function<Fun>(&mut self, mut function: Fun) -> &mut Self
    where
        Fun: Function<P, B>,
    {
        let backend = self.backend.clone();
        let module_path = self.module_path.clone();
        let fun = move |value| {
            let fun = &mut function;
            let module_path = &module_path;
            backend.with_mut(move |mut backend| {
                let source = notify::Source {
                    module_path,
                    action_name: Some(Fun::NAME),
                };
                let args = match backend
                    .deserialize::<Fun::Args>(value)
                    .into_result()
                {
                    Ok(args) => args,
                    Err(err) => {
                        backend.emit_err::<P, _>(source, err);
                        return None;
                    },
                };
                let mut action_ctx = ActionCtx::new(
                    NeovimCtx::new(backend.as_mut(), module_path),
                    Fun::NAME,
                );
                let ret = match fun.call(args, &mut action_ctx).into_result() {
                    Ok(ret) => ret,
                    Err(err) => {
                        backend.emit_err::<P, _>(source, err);
                        return None;
                    },
                };
                match backend.serialize(&ret).into_result() {
                    Ok(ret) => Some(ret),
                    Err(err) => {
                        backend.emit_err::<P, _>(source, err);
                        None
                    },
                }
            })
        };
        self.module_api.add_function(Fun::NAME, fun);
        self
    }

    /// TODO: docs.
    #[inline]
    pub fn with_module<Mod>(&mut self, module: Mod) -> &mut Self
    where
        Mod: Module<P, B>,
    {
        let mut module_api = self.module_api.as_submodule::<Mod>();
        self.module_path.push(Mod::NAME);
        let mut api_ctx = ApiCtx::new(
            &mut module_api,
            self.command_builder.add_module::<Mod>(),
            self.config_builder.add_module::<Mod>(),
            self.module_path,
            self.backend,
        );
        Module::api(&module, &mut api_ctx);
        module_api.finish();
        self.module_path.pop();
        self.config_builder.finish(module);
        self
    }

    #[inline]
    pub(crate) fn new(
        module_api: &'a mut <B::Api<P> as Api<P, B>>::ModuleApi<'b, M>,
        command_builder: CommandBuilder<'a, P, B>,
        config_builder: &'a mut ConfigFnBuilder<P, B>,
        module_path: &'a mut ModulePath,
        backend: &'a BackendHandle<B>,
    ) -> Self {
        Self {
            module_api,
            command_builder,
            config_builder,
            module_path,
            backend,
        }
    }
}

impl<P: Plugin<B>, B: Backend> ConfigFnBuilder<P, B> {
    #[inline]
    pub(crate) fn build(
        mut self,
        backend: BackendHandle<B>,
    ) -> impl FnMut(B::ApiValue) + 'static {
        move |value| {
            backend.with_mut(|backend| {
                let mut module_path = ModulePath::new(self.module_name);
                self.handle(value, &mut module_path, backend)
            });
        }
    }

    #[inline]
    pub(crate) fn finish<M: Module<P, B>>(&mut self, mut module: M) {
        self.config_handler = Box::new(move |value, ctx| {
            let backend = ctx.backend_mut();
            match backend.deserialize::<M::Config>(value).into_result() {
                Ok(config) => module.on_new_config(config, ctx),
                Err(err) => ctx.emit_err(Some(P::CONFIG_FN_NAME), err),
            }
        });
    }

    #[inline]
    pub(crate) fn new<M: Module<P, B>>() -> Self {
        Self {
            module_name: M::NAME,
            config_handler: Box::new(|_, _| {}),
            submodules: Default::default(),
        }
    }

    #[inline]
    fn add_module<M: Module<P, B>>(&mut self) -> &mut Self {
        self.submodules.insert(M::NAME, ConfigFnBuilder::new::<M>())
    }

    #[inline]
    fn handle(
        &mut self,
        mut value: B::ApiValue,
        module_path: &mut ModulePath,
        mut backend: BackendMut<B>,
    ) {
        let mut map_access = match value.map_access() {
            Ok(map_access) => map_access,
            Err(err) => {
                let source = notify::Source {
                    module_path,
                    action_name: Some(P::CONFIG_FN_NAME),
                };
                backend.emit_err::<P, _>(source, err);
                return;
            },
        };
        loop {
            let Some(key) = map_access.next_key() else { break };
            let key_str = match key.as_str() {
                Ok(key) => key,
                Err(err) => {
                    let source = notify::Source {
                        module_path,
                        action_name: Some(P::CONFIG_FN_NAME),
                    };
                    backend.emit_err::<P, _>(source, err);
                    return;
                },
            };
            let Some(submodule) = self.submodules.get_mut(key_str) else {
                continue;
            };
            drop(key);
            let value = map_access.take_next_value();
            module_path.push(submodule.module_name);
            submodule.handle(value, module_path, backend.as_mut());
            module_path.pop();
        }
        drop(map_access);
        let mut ctx = NeovimCtx::new(backend, module_path);
        (self.config_handler)(value, &mut ctx);
    }
}
