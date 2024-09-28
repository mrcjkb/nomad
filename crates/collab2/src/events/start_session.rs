use nomad2::neovim::Neovim;
use nomad2::{Context, Emitter, Event};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StartSession;

impl StartSession {
    pub(crate) const NAME: &str = "start";
}

impl Event<Neovim> for StartSession {
    type Payload = ();
    type SubscribeCtx = ();

    fn subscribe(&mut self, _: Emitter<Self::Payload>, _: &Context<Neovim>) {}
}
