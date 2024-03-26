//! TODO: docs

use core::convert::Infallible;
use std::rc::Rc;

use futures::FutureExt;
use nvim::Object;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::prelude::*;
use crate::serde::{deserialize, serialize};

/// TODO: docs
pub struct Api<M: Module> {
    pub(crate) commands: ModuleCommands,
    pub(crate) functions: Functions,
    pub(crate) module: M,
}

impl<M: Module> Api<M> {
    /// TODO: docs
    #[inline]
    pub fn new(module: M) -> Self {
        Self {
            commands: ModuleCommands::new(M::NAME),
            functions: Functions::default(),
            module,
        }
    }

    /// TODO: docs
    #[inline]
    pub fn with_command<A>(mut self, action: A) -> Self
    where
        A: Action<M, Return = ()>,
        A::Args: TryFrom<CommandArgs>,
        <A::Args as TryFrom<CommandArgs>>::Error: Into<WarningMsg>,
    {
        self.commands.add(action);
        self
    }

    /// TODO: docs
    #[inline]
    pub fn with_function<A>(mut self, action: A) -> Self
    where
        A: Action<M>,
        A::Args: DeserializeOwned,
        A::Return: Serialize,
    {
        self.functions.add(action);
        self
    }
}

/// TODO: docs
#[derive(Default)]
pub(crate) struct Functions {
    functions: nvim::Dictionary,
}

impl Functions {
    /// TODO: docs
    #[inline]
    pub(crate) fn into_dict(self) -> nvim::Dictionary {
        self.functions
    }

    /// TODO: docs
    #[inline]
    fn add<M: Module, A: Action<M>>(&mut self, action: A)
    where
        A::Args: DeserializeOwned,
        A::Return: Serialize,
    {
        let action = Rc::new(action);

        let action = move |args| exec_action(Rc::clone(&action), args);

        self.functions
            .insert(A::NAME.as_str(), nvim::Function::from_fn(action));
    }
}

#[inline]
fn exec_action<M, A>(action: Rc<A>, obj: Object) -> Result<Object, Infallible>
where
    M: Module,
    A: Action<M>,
    A::Args: DeserializeOwned,
    A::Return: Serialize,
{
    enum ActionSyncness<Result> {
        /// The action is synchronous, so the future is guaranteed to resolve
        /// immediately.
        Sync(Result),

        /// The action is asynchronous, so the future may not resolve
        /// immediately.
        Async,
    }

    let future = async move {
        let args = match deserialize::<A::Args>(obj, "args") {
            Ok(args) => args,
            Err(de_err) => return ActionSyncness::Sync(Err(de_err)),
        };

        let future = match action.execute(args).into_enum() {
            MaybeFutureEnum::Ready(res) => {
                return ActionSyncness::Sync(Ok(res
                    .into_result()
                    .map_err(Into::into)))
            },

            MaybeFutureEnum::Future(future) => future,
        };

        if let Err(err) = future.await.into_result() {
            Warning::new()
                .module(M::NAME)
                .action(A::NAME)
                .msg(err.into())
                .print();
        }

        ActionSyncness::Async
    };

    let mut task = spawn(future);

    let Some(res) = (&mut task).now_or_never() else {
        // The action is async and it's not done yet.
        task.detach();
        return Ok(Object::nil());
    };

    let res = match res {
        ActionSyncness::Sync(Ok(res)) => res
            .and_then(|value| serialize(&value, "result").map_err(Into::into)),

        ActionSyncness::Sync(Err(de_err)) => Err(WarningMsg::from(de_err)),

        // The action was async but it resolved on the first poll.
        ActionSyncness::Async => Ok(Object::nil()),
    };

    match res {
        Ok(obj) => Ok(obj),

        Err(warning_msg) => {
            Warning::new()
                .module(M::NAME)
                .action(A::NAME)
                .msg(warning_msg)
                .print();

            Ok(Object::nil())
        },
    }
}
