use core::convert::Infallible;
use std::borrow::Cow;
use std::error::Error;
use std::fs::Metadata;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use futures_lite::Stream;
use nvimx_core::fs::{
    AbsPath,
    DirEntry,
    Fs,
    FsEvent,
    FsNode,
    FsNodeKind,
    FsNodeName,
    Watcher,
};

pub type AnyError = Box<dyn Error + Send + Sync>;

/// TODO: docs.
#[derive(Clone)]
pub struct TestFs {
    inner: Arc<Mutex<TestFsInner>>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestTimestamp(u64);

pub struct TestDirEntry {}

pub struct TestReadDir {}

pub struct TestWatcher {}

struct TestFsInner {
    timestamp: TestTimestamp,
}

impl TestFs {
    #[allow(clippy::unwrap_used)]
    fn with_inner<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut TestFsInner) -> T,
    {
        let mut inner = self.inner.lock().unwrap();
        f(&mut inner)
    }
}

impl Fs for TestFs {
    type Timestamp = TestTimestamp;
    type DirEntry = TestDirEntry;
    type Directory<Path> = ();
    type File<Path> = ();
    type ReadDir = TestReadDir;
    type Watcher = TestWatcher;
    type DirEntryError = Infallible;
    type NodeAtPathError = Infallible;
    type ReadDirError = Infallible;
    type WatchError = Infallible;

    async fn node_at_path<P: AsRef<AbsPath>>(
        &mut self,
        _path: P,
    ) -> Result<Option<FsNode<Self, P>>, Self::NodeAtPathError> {
        todo!()
    }

    fn now(&self) -> Self::Timestamp {
        self.with_inner(|inner| inner.timestamp)
    }

    async fn read_dir<P: AsRef<AbsPath>>(
        &self,
        _dir_path: P,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        todo!()
    }

    async fn watch<P: AsRef<AbsPath>>(
        &mut self,
        _path: P,
    ) -> Result<Self::Watcher, Self::WatchError> {
        todo!()
    }
}

impl DirEntry for TestDirEntry {
    type MetadataError = Infallible;
    type NameError = Infallible;
    type NodeKindError = Infallible;

    async fn metadata(&self) -> Result<Metadata, Self::MetadataError> {
        todo!()
    }

    async fn name(&self) -> Result<Cow<'_, FsNodeName>, Self::NameError> {
        todo!()
    }

    async fn node_kind(&self) -> Result<FsNodeKind, Self::NodeKindError> {
        todo!()
    }
}

impl Stream for TestReadDir {
    type Item = Result<TestDirEntry, Infallible>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

impl Watcher<TestFs> for TestWatcher {
    type Error = Infallible;

    fn register_handler<F>(&mut self, _callback: F)
    where
        F: FnMut(Result<FsEvent<TestFs>, Self::Error>) -> bool + 'static,
    {
        todo!()
    }

    fn watched_path(&self) -> &AbsPath {
        todo!()
    }
}
