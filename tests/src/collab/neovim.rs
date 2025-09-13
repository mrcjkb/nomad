use collab::{CollabEditor, Peer, PeerHandle, PeerId};
use editor::Context;
use neovim::Neovim;
use neovim::tests::NeovimExt;

#[neovim::test]
fn create_peer_tooltip_after_trailing_newline(ctx: &mut Context<Neovim>) {
    ctx.with_borrowed(CollabEditor::on_init);

    let buffer_id = ctx.create_and_focus_scratch_buffer();

    ctx.feedkeys("iHello");

    let peer = Peer {
        id: PeerId::new(1),
        handle: PeerHandle::GitHub("peer1".parse().unwrap()),
    };

    <Neovim as CollabEditor>::create_peer_tooltip(peer, 6, buffer_id, ctx);
}
