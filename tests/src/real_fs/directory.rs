use abs_path::node;
use fs::Directory;
use futures_lite::future;

#[test]
fn create_symlink() {
    future::block_on(async {
        let dir = real_fs::RealFs::default().tempdir().await.unwrap();
        dir.create_symlink(node!("symlink"), "target").await.unwrap();
    });
}
