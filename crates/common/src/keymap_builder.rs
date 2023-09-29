use crate::nvim::{self};
use crate::nvim::{api::opts::*, api::types::Mode, api::Buffer};
use crate::sender::Sender;
use crate::Plugin;

/// TODO: docs
pub struct KeymapBuilder<'a, P: Plugin> {
    /// TODO: docs
    sender: &'a Sender<P::Message>,

    /// TODO: docs
    mode: Option<Mode>,

    /// TODO: docs
    lhs: Option<&'static str>,

    /// TODO: docs
    buffer: Option<Buffer>,

    /// TODO: docs
    opts: SetKeymapOptsBuilder,
}

impl<'a, P: Plugin> KeymapBuilder<'a, P> {
    /// TODO: docs
    pub fn build(&mut self) {
        let opts = std::mem::take(&mut self.opts).build();

        let Some(mode) = self.mode.take() else { panic!("TODO: msg") };

        let Some(lhs) = self.lhs.take() else { panic!("TODO: msg") };

        if let Some(mut buffer) = self.buffer.take() {
            buffer.set_keymap(mode, lhs, "", &opts)
        } else {
            nvim::api::set_keymap(mode, lhs, "", &opts)
        }
        .unwrap();
    }

    /// TODO: docs
    pub fn in_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = Some(buffer);
        self
    }

    /// TODO: docs
    pub fn in_mode(&mut self, mode: Mode) -> &mut Self {
        self.mode = Some(mode);
        self
    }

    /// TODO: docs
    pub fn map(&mut self, lhs: &'static str) -> &mut Self {
        self.lhs = Some(lhs);
        self
    }

    /// TODO: docs
    pub fn new(sender: &'a Sender<P::Message>) -> Self {
        Self {
            sender,
            mode: None,
            lhs: None,
            buffer: None,
            opts: SetKeymapOptsBuilder::default(),
        }
    }

    /// TODO: docs
    pub fn to<F>(&mut self, rhs: F) -> &mut Self
    where
        F: Fn() -> P::Message + 'static,
    {
        let sender = self.sender.clone();

        let rhs = move |()| {
            sender.send(rhs());
            Ok(())
        };

        self.opts.callback(rhs);

        self
    }
}
