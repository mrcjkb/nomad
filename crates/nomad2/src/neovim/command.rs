use core::error::Error;

use super::Neovim;
use crate::Module;

/// TODO: docs.
pub fn command() {}

/// TODO: docs.
pub trait Command: 'static {
    /// TODO: docs.
    const NAME: &'static str;

    /// TODO: docs.
    type Args: TryFrom<CommandArgs>
    where
        <Self::Args as TryFrom<CommandArgs>>::Error: Into<CommandArgsError>;

    /// TODO: docs.
    type Module: Module<Neovim>;
}

/// TODO: docs.
pub struct CommandArgs {}

/// TODO: docs.
pub struct CommandHandle {}

/// TODO: docs.
pub struct CommandEvent {}

/// TODO: docs.
pub struct CommandArgsError {}
