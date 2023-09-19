use std::convert::Infallible;

use common::*;

use crate::schemes;

#[derive(Default)]
pub struct Colorschemes {
    is_disabled: bool,

    sender: LateInit<Sender<Message>>,
}

pub enum Message {
    Close,
    Disable,
    Load(&'static str),
    Open,
}

impl Plugin for Colorschemes {
    const NAME: &'static str = "colorschemes";

    type Message = Message;

    type Config = ();

    type InitError = Infallible;

    type HandleMessageError = Infallible;

    fn init(
        &mut self,
        sender: &Sender<Self::Message>,
    ) -> Result<(), Infallible> {
        self.sender.init(sender.clone());
        Ok(())
    }

    fn init_api(builder: &mut ApiBuilder<'_, Self>) {
        builder.function("open").on_execute(|()| Message::Open).build();

        for colorscheme in schemes::colorschemes() {
            builder
                .function(colorscheme.api_name())
                .on_execute(move |()| Message::Load(colorscheme.api_name()))
                .build();
        }
    }

    fn update_config(&mut self, config: Enable<()>) {
        if !config.enable() {
            self.disable();
        }
    }

    fn handle_message(&mut self, msg: Message) -> Result<(), Infallible> {
        if self.is_disabled {
            return Ok(());
        }

        match msg {
            Message::Close => self.close(),
            Message::Disable => self.disable(),
            Message::Load(colorscheme) => self.load(colorscheme),
            Message::Open => self.open(),
        };

        Ok(())
    }
}

impl Colorschemes {
    fn close(&mut self) {}

    fn load(&mut self, colorscheme: &str) {
        nvim::print!("loading {colorscheme}");
        // self.load(colorscheme)
        // colorscheme.load().unwrap();
    }

    fn disable(&mut self) {
        self.is_disabled = true;
    }

    fn open(&mut self) {}

    #[allow(dead_code)]
    fn send(&mut self, msg: Message) {
        self.sender.send(msg);
    }
}
