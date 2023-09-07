use std::convert::Infallible;

use common::nvim;
use common::*;

use crate::message::Message;

/// TODO: docs
#[derive(Default)]
pub struct Seph {
    /// TODO: docs
    is_open: bool,

    /// TODO: docs
    sender: Option<Sender<Message>>,
}

impl Plugin for Seph {
    const NAME: &'static str = "seph";

    type Message = Message;

    type Config = ();

    type InitError = Infallible;

    type HandleMessageError = Infallible;

    fn init(
        &mut self,
        sender: &Sender<Self::Message>,
    ) -> Result<(), Infallible> {
        self.sender = Some(sender.clone());
        nvim::print!("initialized config");
        Ok(())
    }

    fn init_api(builder: &mut ApiBuilder<'_, Self>) {
        builder.function("open").on_execute(|()| Message::Open).build();

        builder.function("toggle").on_execute(|()| Message::Toggle).build();
    }

    fn init_commands(builder: &mut CommandBuilder<'_, Self>) {
        builder
            .command("Seph")
            .on_execute(|_opts| Message::Open)
            .with_desc("Open a new seph window")
            .build();

        builder
            .command("SephToggle")
            .on_execute(|_opts| Message::Toggle)
            .with_desc("Toggle a seph window")
            .build();
    }

    fn update_config(&mut self, _: Enable<Self::Config>) {
        self.sender.as_ref().unwrap().send(Message::Toggle);
        nvim::print!("updated config");
    }

    fn handle_message(&mut self, msg: Message) -> Result<(), Infallible> {
        match msg {
            Message::Open => self.open(),
            Message::Toggle => self.toggle(),
        };

        Ok(())
    }
}

impl Seph {
    fn close(&mut self) {
        if !self.is_open {
            return;
        }
        self.is_open = false;
        nvim::print!("closed seph window");
    }

    fn open(&mut self) {
        if self.is_open {
            return;
        }
        self.is_open = true;
        nvim::print!("opened seph window");
    }

    fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }
}
