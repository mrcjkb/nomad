mod detach_buffer_actions;
mod register_buffer_actions;
mod session_ctx;

use detach_buffer_actions::DetachBufferActions;
use futures_util::{
    pin_mut,
    select,
    FutureExt,
    Sink,
    SinkExt,
    Stream,
    StreamExt,
};
use nomad::autocmds::{BufAdd, BufUnload};
use nomad::ctx::NeovimCtx;
use nomad::{Action, BufferId, Event, Shared};
use nomad_server::Message;
use register_buffer_actions::RegisterBufferActions;
use session_ctx::SessionCtx;

/// TODO: docs.
pub(crate) struct Session {
    neovim_ctx: NeovimCtx<'static>,
    session_ctx: Shared<SessionCtx>,
}

impl Session {
    pub(crate) fn new() -> Self {
        todo!();
    }

    pub(crate) async fn run<Tx, Rx>(&mut self, remote_tx: Tx, remote_rx: Rx)
    where
        Tx: Sink<Message, Error = core::convert::Infallible>,
        Rx: Stream<Item = Message>,
    {
        let (local_tx, local_rx) = flume::unbounded();

        let mut register_buffer_actions = RegisterBufferActions::new(
            local_tx.clone(),
            self.session_ctx.clone(),
        );

        let detach_buffer_actions = DetachBufferActions::new(
            local_tx.clone(),
            self.session_ctx.clone(),
        );

        for buffer_id in BufferId::opened() {
            let args = nomad::autocmds::BufAddArgs {
                actor_id: nomad::ActorId::unknown(),
                buffer_id,
            };
            register_buffer_actions.execute(args);
        }

        BufAdd::new(register_buffer_actions)
            .register(self.neovim_ctx.reborrow());

        BufUnload::new(detach_buffer_actions)
            .register(self.neovim_ctx.reborrow());

        pin_mut!(remote_rx);
        pin_mut!(remote_tx);

        loop {
            select! {
                remote_message = remote_rx.next().fuse() => {
                    if let Some(remote_message) = remote_message {
                        println!("{:?}", remote_message);
                    }
                },
                local_message = local_rx.recv_async().fuse() => {
                    if let Ok(local_message) = local_message {
                        remote_tx
                            .send(local_message)
                            .await
                            .expect("Infallible");
                    }
                },
            }
        }
    }
}
