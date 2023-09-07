use std::collections::HashMap;
use std::convert::Infallible;

use crate::message::Sender;
use crate::nvim::{lua::Poppable, Dictionary, Function, Object};
use crate::Plugin;

/// TODO: docs
pub struct ApiBuilder<'a, P: Plugin> {
    api: HashMap<&'static str, Object>,
    current: Option<ApiFunction>,
    sender: &'a Sender<P::Message>,
}

struct ApiFunction {
    name: &'static str,
    func: Option<Object>,
}

impl<'a, P: Plugin> ApiBuilder<'a, P> {
    /// TODO: docs
    pub fn new(sender: &'a Sender<P::Message>) -> Self {
        Self { api: HashMap::new(), current: None, sender }
    }

    /// TODO: docs
    pub fn function(
        &mut self,
        name: &'static str,
    ) -> OnExecuteApiBuilder<'a, '_, P> {
        self.current = Some(ApiFunction { name, func: None });
        OnExecuteApiBuilder { builder: self }
    }

    /// TODO: docs
    pub fn build(&mut self) {
        if let Some(func) = self.current.take() {
            if let Some(value) = func.func {
                self.api.insert(func.name, value);
            }
        }
    }

    /// TODO: docs
    pub fn api(self) -> Dictionary {
        Dictionary::from_iter(self.api)
    }
}

pub struct OnExecuteApiBuilder<'a, 'builder, P: Plugin> {
    builder: &'builder mut ApiBuilder<'a, P>,
}

impl<'a, 'builder, P: Plugin> OnExecuteApiBuilder<'a, 'builder, P> {
    /// TODO: docs
    pub fn on_execute<A, F>(self, func: F) -> &'builder mut ApiBuilder<'a, P>
    where
        A: Poppable,
        F: Fn(A) -> P::Message + 'static,
    {
        let sender = self.builder.sender.clone();
        let func = move |args| {
            sender.send(func(args));
            Ok::<_, Infallible>(())
        };
        let func = Function::from_fn(func);
        self.builder.current.as_mut().unwrap().func = Some(Object::from(func));
        self.builder
    }
}
