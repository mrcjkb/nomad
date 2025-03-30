use std::sync::Mutex;

use abs_path::{AbsPath, AbsPathBuf};
use ed::AsyncCtx;
use ed::fs::{
    self,
    Directory,
    DirectoryEvent,
    File,
    FileEvent,
    Fs,
    FsNode,
    NodeCreation,
    NodeMetadataError,
    Symlink,
};
use futures_util::select;
use futures_util::stream::{SelectAll, StreamExt};
use fxhash::{FxHashMap, FxHashSet};
use walkdir::Filter;

use crate::CollabBackend;
use crate::event::Event;

type DirEventStream<Fs> =
    <<Fs as fs::Fs>::Directory as Directory>::EventStream;

type FileEventStream<Fs> = <<Fs as fs::Fs>::File as File>::EventStream;

/// TODO: docs.
pub(crate) struct EventStream<
    B: CollabBackend,
    FsFilter = <B as CollabBackend>::FsFilter,
> {
    buffer_ids: FxHashMap<<B::Fs as fs::Fs>::NodeId, B::BufferId>,
    directory_streams: SelectAll<DirEventStream<B::Fs>>,
    file_streams: SelectAll<FileEventStream<B::Fs>>,
    fs_filter: FsFilter,
    root_id: <B::Fs as fs::Fs>::NodeId,
    root_path: AbsPathBuf,
    saved_buffers: FxHashSet<B::BufferId>,
}

/// TODO: docs.
pub(crate) struct EventStreamBuilder<B: CollabBackend> {
    stream: Mutex<EventStream<B, ()>>,
}

/// TODO: docs.
pub(crate) enum EventStreamError<B: CollabBackend> {
    FollowSymlink(<<B::Fs as fs::Fs>::Symlink as Symlink>::FollowError),
    FsFilter(<B::FsFilter as Filter<B::Fs>>::Error),
    Metadata(NodeMetadataError<B::Fs>),
    NodeAtPath(<B::Fs as fs::Fs>::NodeAtPathError),
}

impl<B: CollabBackend> EventStream<B> {
    pub(crate) fn builder(project_root: AbsPathBuf) -> EventStreamBuilder<B> {
        EventStreamBuilder {
            stream: Mutex::new(EventStream {
                buffer_ids: Default::default(),
                directory_streams: SelectAll::new(),
                file_streams: SelectAll::new(),
                fs_filter: (),
                root_id: todo!(),
                root_path: project_root,
                saved_buffers: Default::default(),
            }),
        }
    }

    pub(crate) async fn next(
        &mut self,
        ctx: &mut AsyncCtx<'_, B>,
    ) -> Result<Event<B>, EventStreamError<B>> {
        loop {
            let event = select! {
                dir_event = self.directory_streams.select_next_some() => {
                    Event::Directory(dir_event)
                },
                file_event = self.file_streams.select_next_some() => {
                    Event::File(file_event)
                },
            };

            self.on_event(&event, ctx).await?;

            if self.should_emit(&event) {
                return Ok(event);
            }
        }
    }

    async fn on_event(
        &mut self,
        event: &Event<B>,
        ctx: &mut AsyncCtx<'_, B>,
    ) -> Result<(), EventStreamError<B>> {
        match event {
            Event::Directory(DirectoryEvent::Creation(creation)) => {
                self.on_node_creation(creation, ctx).await?;
            },
            Event::Directory(DirectoryEvent::Move(r#move)) => {
                if !r#move.new_path.starts_with(&self.root_path) {
                    todo!("drop dir's stream");
                }
            },
            Event::File(FileEvent::Move(r#move)) => {
                if !r#move.new_path.starts_with(&self.root_path) {
                    todo!("drop file's stream");
                }
            },
            _ => {},
        }
        Ok(())
    }

    async fn on_node_creation(
        &mut self,
        creation: &fs::NodeCreation<B::Fs>,
        ctx: &mut AsyncCtx<'_, B>,
    ) -> Result<(), EventStreamError<B>> {
        let Some(node) = ctx
            .fs()
            .node_at_path(&creation.node_path)
            .await
            .map_err(EventStreamError::NodeAtPath)?
        else {
            // The node must've already been deleted.
            return Ok(());
        };

        let meta = node.meta().await.map_err(EventStreamError::Metadata)?;

        let parent_path = creation.node_path.parent().expect("has a parent");

        if self
            .fs_filter
            .should_filter(parent_path, &meta)
            .await
            .map_err(EventStreamError::FsFilter)?
        {
            return Ok(());
        }

        match node {
            FsNode::File(file) => {
                todo!()
            },
            FsNode::Directory(dir) => {
                self.directory_streams.push(dir.watch().await);
            },
            FsNode::Symlink(_) => {},
        }

        Ok(())
    }

    fn should_emit(&mut self, event: &Event<B>) -> bool {
        match event {
            Event::Directory(DirectoryEvent::Deletion(deletion))
            | Event::File(FileEvent::Deletion(deletion)) => {
                deletion.node_id == deletion.deletion_root_id
                    || deletion.node_id == self.root_id
            },

            Event::Directory(DirectoryEvent::Move(r#move))
            | Event::File(FileEvent::Move(r#move)) => {
                r#move.node_id == r#move.r#move_root_id
                    || r#move.node_id == self.root_id
            },

            Event::File(FileEvent::Modification(modification)) => self
                .buffer_ids
                .get(&modification.file_id)
                .map(|buf_id| self.saved_buffers.remove(buf_id))
                .unwrap_or(true),

            _ => true,
        }
    }
}

impl<B: CollabBackend> EventStreamBuilder<B> {
    pub(crate) fn build(self, fs_filter: B::FsFilter) -> EventStream<B> {
        let stream = self.stream.into_inner().expect("poisoned");
        let EventStream { directory_streams, .. } = stream;
        todo!();
        // EventStream { directory_streams, fs_filter }
    }

    pub(crate) async fn push_node(
        &self,
        node: &FsNode<B::Fs>,
        ctx: &AsyncCtx<'_, B>,
    ) -> Result<(), EventStreamError<B>> {
        match node {
            FsNode::Directory(dir) => {
                self.push_directory(dir).await;
                Ok(())
            },
            FsNode::File(file) => self.push_file(file, ctx).await,
            FsNode::Symlink(symlink) => self.push_symlink(symlink, ctx).await,
        }
    }

    async fn push_directory(&self, dir: &<B::Fs as fs::Fs>::Directory) {
        let stream = dir.watch().await;
        self.stream.lock().expect("poisoned").directory_streams.push(stream);
    }

    async fn push_file(
        &self,
        _file: &<B::Fs as fs::Fs>::File,
        _ctx: &AsyncCtx<'_, B>,
    ) -> Result<(), EventStreamError<B>> {
        todo!()
    }

    async fn push_symlink(
        &self,
        symlink: &<B::Fs as fs::Fs>::Symlink,
        ctx: &AsyncCtx<'_, B>,
    ) -> Result<(), EventStreamError<B>> {
        // FIXME: we should add a watcher on the symlink itself to react to its
        // deletion.

        let Some(node) = symlink
            .follow_recursively()
            .await
            .map_err(EventStreamError::FollowSymlink)?
        else {
            return Ok(());
        };

        match node {
            FsNode::Directory(dir) => {
                self.push_directory(&dir).await;
                Ok(())
            },
            FsNode::File(file) => self.push_file(&file, ctx).await,
            FsNode::Symlink(_) => unreachable!(
                "following recursively must resolve to a File or Directory"
            ),
        }
    }
}
