use core::ops::Range;

use nohash::IntMap as NoHashMap;
use nvim_oxi::api::{self, opts};

use crate::ctx::NeovimCtx;
use crate::{BufferId, ByteOffset, Shared};

pub(crate) struct DecorationProvider {
    selections: NoHashMap<BufferId, SelectionInfos>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct NamespaceId(u32);

struct SelectionInfos {
    range: Shared<Option<Range<ByteOffset>>>,
}

impl DecorationProvider {
    pub(crate) fn new(ctx: NeovimCtx<'static>) -> Self {
        let opts = opts::DecorationProviderOpts::builder()
            .on_start({
                let ctx = ctx.clone();
                move |_| ctx.with_decoration_provider(|this| this.on_start())
            })
            .on_buf({
                let ctx = ctx.clone();
                move |(_, nvim_buf)| {
                    let buffer_id = BufferId::new(nvim_buf);
                    ctx.with_decoration_provider(|this| {
                        this.on_buffer(buffer_id)
                    })
                }
            })
            .build();

        api::set_decoration_provider(ctx.namespace_id().into_u32(), &opts)
            .expect("all arguments are valid");

        Self { selections: NoHashMap::default() }
    }

    fn on_start(&self) -> bool {
        todo!();
    }

    fn on_buffer(&mut self, _buffer_id: BufferId) {
        todo!();
    }
}

impl NamespaceId {
    pub(crate) fn new(namespace_name: &str) -> Self {
        Self(api::create_namespace(namespace_name))
    }

    fn into_u32(self) -> u32 {
        self.0
    }
}
