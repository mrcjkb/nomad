use core::ops::Range;

use nohash::IntMap as NoHashMap;
use nvim_oxi::api::{self, opts};

use crate::ctx::{BufferCtx, NeovimCtx};
use crate::diagnostics::HighlightGroup;
use crate::{BufferId, ByteOffset, Shared};

/// TODO: docs.
pub struct Selection {
    infos: Shared<SelectionInfos>,
}

pub(crate) struct DecorationProvider {
    namespace_id: NamespaceId,
    selections: NoHashMap<BufferId, Vec<Shared<SelectionInfos>>>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct NamespaceId(u32);

struct SelectionInfos {
    range: Range<ByteOffset>,
    highlight_group: HighlightGroup,
    extmark_id: ExtmarkId,
    was_dropped: bool,
}

#[derive(Debug, Copy, Clone)]
struct ExtmarkId(u32);

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
                    let buffer_ctx =
                        ctx.reborrow().into_buffer(buffer_id).expect(
                            "`on_buf` was just called, so the buffer must \
                             exist",
                        );
                    ctx.with_decoration_provider(|this| {
                        this.on_buffer(buffer_ctx)
                    })
                }
            })
            .build();

        let namespace_id = ctx.namespace_id();

        api::set_decoration_provider(namespace_id.into_u32(), &opts)
            .expect("all arguments are valid");

        Self { namespace_id, selections: NoHashMap::default() }
    }

    fn on_start(&self) -> bool {
        // The return value informs Neovim whether to execute the various
        // callbacks for the current redraw cycle.
        !self.selections.is_empty()
    }

    fn on_buffer(&mut self, buffer_ctx: BufferCtx<'_>) {
        self.update_selections(buffer_ctx);
    }

    fn update_selections(&mut self, buffer_ctx: BufferCtx<'_>) {
        let buffer_id = buffer_ctx.buffer_id();
        let Some(selection_infos) = self.selections.get_mut(&buffer_id) else {
            return;
        };
        let mut idx = 0;
        loop {
            let Some(selection_info) = selection_infos.get_mut(idx) else {
                break;
            };
            let was_dropped = selection_info.with_mut(|infos| {
                if infos.was_dropped {
                    return true;
                }
                let point_range =
                    buffer_ctx.point_range_of_byte_range(infos.range.clone());
                let opts = opts::SetExtmarkOpts::builder()
                    .id(infos.extmark_id.into_u32())
                    .end_row(point_range.end.line_idx)
                    .end_col(point_range.end.byte_offset.into())
                    .hl_group(infos.highlight_group.as_str())
                    .ephemeral(true)
                    .build();
                buffer_id
                    .as_nvim()
                    .set_extmark(
                        self.namespace_id.into_u32(),
                        point_range.start.line_idx,
                        point_range.start.byte_offset.into(),
                        &opts,
                    )
                    .expect("all arguments are valid");
                false
            });
            if was_dropped {
                selection_infos.remove(idx);
            } else {
                idx += 1;
            }
        }
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

impl ExtmarkId {
    fn into_u32(self) -> u32 {
        self.0
    }
}
