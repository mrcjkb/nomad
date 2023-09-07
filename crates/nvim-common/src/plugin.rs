use std::error::Error;

use serde::de::DeserializeOwned;

use crate::*;

/// TODO: docs
pub trait Plugin: Default + 'static {
    /// TODO: docs
    const NAME: &'static str;

    /// TODO: docs
    type Message: 'static;

    /// TODO: docs
    type Config: DeserializeOwned + 'static;

    /// TODO: docs
    type InitError: Error + 'static;

    /// TODO: docs
    type HandleMessageError: Error + 'static;

    /// TODO: docs
    fn init_api(builder: &mut ApiBuilder<'_, Self>);

    /// TODO: docs
    fn init_commands(builder: &mut CommandBuilder<'_, Self>);

    /// TODO: docs
    fn init(
        &mut self,
        sender: &Sender<Self::Message>,
    ) -> Result<(), Self::InitError>;

    /// TODO: docs
    fn handle(
        &mut self,
        msg: Self::Message,
    ) -> Result<(), Self::HandleMessageError>;

    /// TODO: docs
    fn config(&mut self, config: Enable<Self::Config>);
}
