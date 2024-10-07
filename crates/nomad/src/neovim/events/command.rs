use core::cmp::Ordering;
use core::marker::PhantomData;

use crate::neovim::command::{Command, OnExecute};
use crate::neovim::Neovim;
use crate::{Context, Emitter, Event, Shared};

/// TODO: docs.
pub struct CommandEvent<T> {
    pub(in crate::neovim) module_name: &'static str,
    pub(in crate::neovim) command_name: &'static str,
    pub(in crate::neovim) on_execute_buf: Shared<Option<OnExecute>>,
    pub(in crate::neovim) ty: PhantomData<T>,
}

impl<T: Command> Event<Neovim> for CommandEvent<T> {
    type Payload = T::Args;
    type SubscribeCtx = ();

    fn subscribe(&mut self, emitter: Emitter<T::Args>, _: &Context<Neovim>) {
        let on_execute = Box::new(move |mut args| {
            let args = T::Args::try_from(&mut args).map_err(Into::into)?;
            emitter.send(args);
            Ok(())
        });

        self.on_execute_buf.with_mut(|buf| {
            *buf = Some(on_execute);
        });
    }
}

impl<T> PartialEq for CommandEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for CommandEvent<T> {}

impl<T> PartialOrd for CommandEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for CommandEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.module_name.cmp(other.module_name) {
            Ordering::Equal => self.command_name.cmp(other.command_name),
            ord => ord,
        }
    }
}
