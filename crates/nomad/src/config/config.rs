use core::cell::{OnceCell, RefCell};
use std::collections::HashMap;

use serde::de::{self, Deserialize};

use super::EnableConfig;
use crate::ctx::{Ctx, Set};
use crate::module::{Module, ModuleId};
use crate::nvim::{serde::Deserializer, Function, Object};

thread_local! {
    /// TODO: docs
    static DESERIALIZERS: ConfigDeserializers
        = const { ConfigDeserializers::new() };
}

/// TODO: docs
pub(crate) fn config() -> Function<Object, ()> {
    Function::from_fn(|object| {
        let deserializer = Deserializer::new(object);
        UpdateConfigs::deserialize(deserializer).unwrap();
        Ok::<_, core::convert::Infallible>(())
    })
}

/// TODO: docs
#[inline]
pub(crate) fn with_module<M>(set_config: Set<EnableConfig<M>>, ctx: Ctx)
where
    M: Module,
{
    DESERIALIZERS.with(|deserializers| {
        let deserializer = ConfigDeserializer::new(set_config, ctx);
        deserializers.insert(M::NAME.id(), deserializer)
    });
}

/// TODO: docs
struct ConfigDeserializers {
    deserializers: OnceCell<RefCell<HashMap<ModuleId, ConfigDeserializer>>>,
}

impl ConfigDeserializers {
    /// TODO: docs
    #[inline]
    fn insert(&self, id: ModuleId, deserializer: ConfigDeserializer) {
        self.with_map(|map| map.insert(id, deserializer));
    }

    /// TODO: docs
    const fn new() -> Self {
        Self { deserializers: OnceCell::new() }
    }

    /// TODO: docs
    #[inline]
    fn with_map<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut HashMap<ModuleId, ConfigDeserializer>) -> R,
    {
        let inner = self.deserializers.get_or_init(RefCell::default);
        let map = &mut *inner.borrow_mut();
        f(map)
    }
}

/// TODO: docs
struct ConfigDeserializer {
    deserializer: Box<dyn Fn(Object) + 'static>,
}

impl ConfigDeserializer {
    /// TODO: docs
    #[inline]
    fn deserialize(&self, config: Object) {
        (self.deserializer)(config);
    }

    /// TODO: docs
    #[inline]
    fn new<M: Module>(set_config: Set<EnableConfig<M>>, ctx: Ctx) -> Self {
        let deserializer = move |config: Object| {
            let deserializer = Deserializer::new(config);
            let config = match EnableConfig::<M>::deserialize(deserializer) {
                Ok(config) => config,
                Err(_err) => return,
            };
            ctx.with_set(|set_ctx| set_config.set(config, set_ctx));
        };

        Self { deserializer: Box::new(deserializer) }
    }
}

/// TODO: docs
struct UpdateConfigs;

impl<'de> Deserialize<'de> for UpdateConfigs {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(UpdateConfigsVisitor)
    }
}

struct UpdateConfigsVisitor;

impl<'de> de::Visitor<'de> for UpdateConfigsVisitor {
    type Value = UpdateConfigs;

    #[inline]
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a dictionary")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        // We store the module names and their configs because we want to make
        // sure that the map deserializes correctly before deserializing the
        // individual configs for each module.
        //
        // Not doing this could cause only some of the configs to be updated.
        // For example:
        //
        // ```lua
        // nomad.config({
        //   foo = { .. },
        //   "hello",
        //   bar = { .. },
        // })
        // ```
        //
        // In this case, the `foo` module would be updated, but the `bar`
        // module wouldn't because the `config` function would return an error
        // when it gets to `"hello"`.

        let mut buffer = Vec::new();

        while let Some(module_name) = map.next_key::<String>()? {
            let module_config = map.next_value::<Object>()?;
            buffer.push((module_name, module_config));
        }

        for (module_name, module_config) in buffer {
            if let Err(_err) = update_config(module_name, module_config) {
                todo!();
            }
        }

        Ok(UpdateConfigs)
    }
}

/// TODO: docs
struct InvalidModule(String);

/// TODO: docs
#[inline]
fn update_config(
    module_name: String,
    module_config: Object,
) -> Result<(), InvalidModule> {
    let module_id = ModuleId::from_module_name(&module_name);

    DESERIALIZERS.with(move |deserializers| {
        deserializers.with_map(|map| {
            map.get(&module_id)
                .ok_or(InvalidModule(module_name))
                .map(|des| des.deserialize(module_config))
        })
    })
}
