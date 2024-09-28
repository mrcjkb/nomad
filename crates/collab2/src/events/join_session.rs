use collab_server::SessionId;
use nomad2::neovim::Neovim;
use nomad2::{Context, Emitter, Event};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct JoinSession;

impl JoinSession {
    pub(crate) const NAME: &str = "join";
}

impl Event<Neovim> for JoinSession {
    type Payload = SessionId;
    type SubscribeCtx = ();

    fn subscribe(&mut self, _: Emitter<Self::Payload>, _: &Context<Neovim>) {}
}
