use abs_path::{AbsPath, AbsPathBuf};
use ed::{AsyncCtx, fs};
use walkdir::DirEntry;

use crate::CollabBackend;
use crate::event::Event;

/// TODO: docs.
#[derive(Clone)]
pub(crate) struct EventStream<B: CollabBackend> {
    _fs: core::marker::PhantomData<B>,
}

/// TODO: docs.
pub(crate) struct EventStreamBuilder<B> {
    _project_root: AbsPathBuf,
    _fs: core::marker::PhantomData<B>,
}

/// TODO: docs.
pub(crate) enum PushError<Fs: fs::Fs> {
    Todo(core::marker::PhantomData<Fs>),
}

impl<B: CollabBackend> EventStream<B> {
    pub(crate) fn builder(project_root: &AbsPath) -> EventStreamBuilder<B> {
        EventStreamBuilder {
            _project_root: project_root.to_owned(),
            _fs: core::marker::PhantomData,
        }
    }

    pub(crate) async fn next(&mut self, _ctx: &mut AsyncCtx<'_, B>) -> Event {
        todo!()
    }
}

impl<B: CollabBackend> EventStreamBuilder<B> {
    pub(crate) fn build(self, _fs_filter: B::FsFilter) -> EventStream<B> {
        EventStream { _fs: self._fs }
    }

    pub(crate) async fn push_node(
        &self,
        _dir_path: &AbsPath,
        _node: DirEntry<B::Fs>,
        _ctx: &AsyncCtx<'_, B>,
    ) -> Result<(), PushError<B::Fs>> {
        todo!()
    }
}
