use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;
use ed::fs::Directory;

pub(crate) fn benches(group: &mut BenchmarkGroup<'_, WallTime>) {
    #[cfg(feature = "neovim-repo")]
    read_neovim_from_fs::mock_fs(group);
}

#[cfg(feature = "neovim-repo")]
mod read_neovim_from_fs {
    use auth::Auth;
    use collab::mock::CollabMock;
    use collab::{Collab, CollabBackend};
    use criterion::BenchmarkId;
    use ed::fs::Fs;
    use ed::fs::os::{OsDirectory, OsFs};
    use futures_lite::future;
    use mock::{BackendExt, Mock};

    use super::*;

    pub(super) fn mock_fs(group: &mut BenchmarkGroup<'_, WallTime>) {
        let fs = mock::fs! {};

        // Replicate the Neovim repo into the root of the mock filesystem.
        future::block_on(async {
            fs.root().replicate_from(&neovim_repo()).await.unwrap();
        });

        CollabMock::<Mock>::default().block_on(async move |ctx| {
            bench_read_project(fs.root().path(), "mock_fs", ctx, group);
        });
    }

    fn neovim_repo() -> OsDirectory {
        future::block_on(async {
            OsFs::default()
                .node_at_path(crate::generated::collab::NEOVIM_REPO_PATH)
                .await
                .unwrap()
                .unwrap()
                .unwrap_directory()
        })
    }

    fn bench_read_project<B: CollabBackend>(
        project_root: &abs_path::AbsPath,
        fs_name: &str,
        ctx: &mut ed::AsyncCtx<B>,
        group: &mut BenchmarkGroup<'_, WallTime>,
    ) {
        let bench_id = BenchmarkId::new(
            "start",
            format_args!("read_neovim_from_{fs_name}"),
        );

        let start = Collab::<B>::from(&Auth::logged_in("peer")).start();

        group.bench_function(bench_id, |b| {
            b.iter(|| {
                future::block_on(async {
                    start.read_project(project_root, ctx).await.unwrap();
                })
            });
        });
    }
}
