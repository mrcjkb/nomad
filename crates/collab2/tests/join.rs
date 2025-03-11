#![allow(missing_docs)]

use auth::Auth;
use collab2::Collab;
use collab2::backend::test::{CollabTestBackend, CollabTestServer, SessionId};
use futures_lite::future::{self, FutureExt};
use nvimx2::action::AsyncAction;
use nvimx2::command::Parse;
use nvimx2::fs::{AbsPathBuf, Fs};
use nvimx2::tests::{self, BackendExt, TestBackend};

#[test]
fn replicate_simple_project() {
    let fs1 = tests::fs! {
        "foo": {
            "world.txt": "Hello, world!",
            "mars.txt": "Hello, mars!",
        },
    };

    let server = CollabTestServer::default();

    let peer1 = CollabTestBackend::new(TestBackend::new(fs1.clone()))
        .with_home_dir(AbsPathBuf::root())
        .with_server(&server);

    let peer2 = CollabTestBackend::<TestBackend>::default()
        .with_default_dir_for_remote_projects(path("/remote"))
        .with_server(&server);

    let run_peer1 = peer1.run(async |ctx| {
        let collab = Collab::from(&Auth::dummy("peer1"));
        ctx.focus_buffer_at(&path("/foo/mars.txt")).unwrap();
        collab.start().call((), ctx).await.unwrap();
    });

    let run_peer2 = peer2.run(async move |ctx| {
        let collab = Collab::from(&Auth::dummy("peer2"));
        collab.join().call(Parse(SessionId(1)), ctx).await.unwrap();
        let fs2 = ctx.fs();
        assert_eq!(
            fs1.node_at_path(path("/foo")).await.unwrap().unwrap(),
            fs2.node_at_path(path("/remote/foo")).await.unwrap().unwrap(),
        );
    });

    let run_test = async {
        run_peer1.await;
        run_peer2.await;
    };

    future::block_on(run_test.race(server.run()));
}

fn path(path: &str) -> AbsPathBuf {
    path.parse().unwrap()
}
