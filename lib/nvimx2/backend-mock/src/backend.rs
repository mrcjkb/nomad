use fxhash::FxHashMap;
use nvimx_core::backend::{ApiValue, Backend};
use nvimx_core::fs::{AbsPath, FsNode};
use nvimx_core::notify::MaybeResult;
use serde::{Deserialize, Serialize};

use crate::api::Api;
use crate::buffer::{Buffer, BufferId};
use crate::emitter::Emitter;
use crate::executor::Executor;
use crate::fs::MockFs;
use crate::serde::{DeserializeError, SerializeError};

/// TODO: docs.
pub struct Mock {
    buffers: FxHashMap<BufferId, Buffer>,
    current_buffer: Option<BufferId>,
    emitter: Emitter,
    executor: Executor,
    fs: MockFs,
    next_buffer_id: BufferId,
}

impl Mock {
    pub fn new(fs: MockFs) -> Self {
        Self {
            buffers: Default::default(),
            current_buffer: None,
            emitter: Default::default(),
            executor: Default::default(),
            fs,
            next_buffer_id: BufferId(1),
        }
    }

    fn buffer_at(&self, path: &AbsPath) -> Option<&Buffer> {
        self.buffers.values().find(|buf| path.as_str() == buf.name)
    }

    #[track_caller]
    fn buffer_mut(&mut self, id: BufferId) -> &mut Buffer {
        self.buffers.get_mut(&id).expect("buffer exists")
    }

    #[track_caller]
    fn open_buffer(&mut self, path: &AbsPath) -> &mut Buffer {
        assert!(self.buffer_at(path).is_none());

        let file =
            match self.fs.node_at_path_sync(path).expect("no file at path") {
                FsNode::File(file) => file,
                _ => todo!(),
            };

        let contents =
            str::from_utf8(&file.read_sync().expect("just got file"))
                .expect("file is not valid UTF-8")
                .into();

        let buffer = Buffer {
            contents,
            id: self.next_buffer_id.post_inc(),
            name: path.to_string(),
        };

        let buffer_id = buffer.id;

        self.buffers.insert(buffer.id, buffer);

        self.buffer_mut(buffer_id)
    }
}

impl Backend for Mock {
    const REINSTATE_PANIC_HOOK: bool = true;

    type Api = Api;
    type Buffer<'a> = &'a mut Buffer;
    type BufferId = BufferId;
    type LocalExecutor = Executor;
    type BackgroundExecutor = Executor;
    type Fs = MockFs;
    type Emitter<'this> = &'this mut Emitter;
    type SerializeError = SerializeError;
    type DeserializeError = DeserializeError;

    fn buffer(&mut self, id: BufferId) -> Option<Self::Buffer<'_>> {
        self.buffers.get_mut(&id)
    }

    fn buffer_ids(&mut self) -> impl Iterator<Item = BufferId> + use<> {
        self.buffers.keys().copied().collect::<Vec<_>>().into_iter()
    }

    fn current_buffer(&mut self) -> Option<Self::Buffer<'_>> {
        self.current_buffer.map(|id| self.buffer_mut(id))
    }

    fn fs(&mut self) -> Self::Fs {
        self.fs.clone()
    }

    fn emitter(&mut self) -> Self::Emitter<'_> {
        &mut self.emitter
    }

    fn local_executor(&mut self) -> &mut Self::LocalExecutor {
        &mut self.executor
    }

    fn focus_buffer_at(&mut self, path: &AbsPath) -> Option<Self::Buffer<'_>> {
        let buf_id = self
            .buffer_at(path)
            .map(|buf| buf.id)
            .unwrap_or_else(|| self.open_buffer(path).id);
        self.current_buffer = Some(buf_id);
        Some(self.buffer_mut(buf_id))
    }

    fn background_executor(&mut self) -> &mut Self::BackgroundExecutor {
        &mut self.executor
    }

    fn serialize<T>(
        &mut self,
        value: &T,
    ) -> impl MaybeResult<ApiValue<Self>, Error = Self::SerializeError>
    where
        T: ?Sized + Serialize,
    {
        crate::serde::serialize(value)
    }

    fn deserialize<'de, T>(
        &mut self,
        value: ApiValue<Self>,
    ) -> impl MaybeResult<T, Error = Self::DeserializeError>
    where
        T: Deserialize<'de>,
    {
        crate::serde::deserialize(value)
    }
}

impl Default for Mock {
    fn default() -> Self {
        Self::new(MockFs::default())
    }
}
