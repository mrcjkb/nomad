//! TODO: docs

use core::convert::Infallible;

use nvim::{self, Object};
use serde::{de::Deserialize, ser::Serialize};

use crate::prelude::*;

/// TODO: docs
pub struct Api<M: Module> {
    pub(crate) functions: Functions,
    pub(crate) module: M,
}

impl<M: Module> Api<M> {
    /// TODO: docs
    #[inline]
    pub fn new(module: M) -> Self {
        Self { functions: Functions::default(), module }
    }

    /// TODO: docs
    #[inline]
    pub fn with_command<A>(self, _action: A) -> Self
    where
        A: Action<M, Return = ()>,
        A::Args: TryFrom<CommandArgs>,
        <A::Args as TryFrom<CommandArgs>>::Error: Into<WarningMsg>,
    {
        self
    }

    /// TODO: docs
    #[inline]
    pub fn with_function<A>(mut self, action: A) -> Self
    where
        A: Action<M>,
    {
        self.functions.push(action);
        self
    }
}

type Function = Box<dyn Fn(Object, &mut SetCtx) -> Result<Object, Infallible>>;

/// TODO: docs
#[derive(Default)]
pub(crate) struct Functions {
    functions: Vec<(ActionName, Function)>,
}

impl Functions {
    /// TODO: docs
    #[inline]
    pub(crate) fn into_iter(
        self,
        ctx: Ctx,
    ) -> impl Iterator<Item = (ActionName, nvim::Function<Object, Object>)>
    {
        self.functions.into_iter().map(move |(name, function)| {
            let ctx = ctx.clone();

            let function = nvim::Function::from_fn(move |object: Object| {
                ctx.with_set(|set_ctx| function(object, set_ctx))
            });

            (name, function)
        })
    }

    /// TODO: docs
    #[inline]
    fn push<M: Module, A: Action<M>>(&mut self, action: A) {
        let function = move |args: Object, ctx: &mut SetCtx| {
            let deserializer = nvim::serde::Deserializer::new(args);
            let args = A::Args::deserialize(deserializer).unwrap();
            let ret = match action.execute(args, ctx).into_result() {
                Ok(v) => v,
                Err(err) => {
                    Warning::new()
                        .module(M::NAME)
                        .action(A::NAME)
                        .msg(err.into())
                        .print();
                    return Ok(Object::nil());
                },
            };
            let serializer = nvim::serde::Serializer::new();
            match ret.serialize(serializer) {
                Ok(obj) => Ok(obj),
                Err(_err) => todo!(),
            }
        };

        self.functions.push((A::NAME, Box::new(function)));
    }
}

/// TODO: docs
pub struct CommandArgs {
    args: Vec<String>,
}

impl CommandArgs {
    fn into_iter(self) -> impl Iterator<Item = String> {
        self.args.into_iter()
    }

    fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    fn len(&self) -> usize {
        self.args.len()
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

impl From<Infallible> for WarningMsg {
    #[inline]
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible can't be constructed")
    }
}
