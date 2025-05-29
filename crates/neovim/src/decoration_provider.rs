use core::ops::Range;

use compact_str::CompactString;
use ed::Shared;
use nohash::IntMap as NoHashMap;
use slotmap::SlotMap;

use crate::buffer::{BufferId, Point};
use crate::oxi::api;

/// TODO: docs.
pub struct HighlightRange {
    decoration_provider: DecorationProvider,
    buffer_id: BufferId,
    range_id: slotmap::DefaultKey,
}

pub(crate) struct DecorationProvider {
    inner: Shared<DecorationProviderInner>,
}

struct DecorationProviderInner {
    namespace_id: u32,
    highlight_ranges: NoHashMap<BufferId, HighlightRanges>,
}

/// The highlight ranges to be drawn in a given buffer.
struct HighlightRanges {
    buffer_id: BufferId,
    inner: SlotMap<slotmap::DefaultKey, HighlightRangeInner>,
}

struct HighlightRangeInner {
    extmark_id: u32,
    highlight_group_name: CompactString,
    point_range: Range<Point>,
}

impl HighlightRange {
    #[inline]
    fn with_inner<T>(
        &self,
        fun: impl FnOnce(&mut HighlightRangeInner) -> T,
    ) -> T {
        self.decoration_provider.inner.with_mut(|decoration_provider| {
            let inner = decoration_provider
                .highlight_ranges
                .get_mut(&self.buffer_id)
                .expect("not removed until all ranges buffer are dropped")
                .inner
                .get_mut(self.range_id)
                .expect("not removed until this range is dropped");

            fun(inner)
        })
    }
}

impl DecorationProvider {
    #[inline]
    pub(crate) fn new(namespace_name: &str) -> Self {
        let namespace_id = api::create_namespace(namespace_name);

        let this = Self {
            inner: Shared::new(DecorationProviderInner {
                namespace_id,
                highlight_ranges: Default::default(),
            }),
        };

        let opts = api::opts::DecorationProviderOpts::builder()
            .on_start(this.on_start())
            .on_buf(this.on_buf())
            .build();

        api::set_decoration_provider(namespace_id, &opts)
            .expect("couldn't set decoration provider");

        this
    }

    #[inline]
    fn on_start(
        &self,
    ) -> impl Fn(api::opts::OnStartArgs) -> api::opts::DontSkipRedrawCycle + 'static
    {
        let inner = self.inner.clone();

        move |_args| {
            inner.with(|inner| {
                // The return value informs Neovim whether to execute the
                // various callbacks for the current redraw cycle.
                !inner.highlight_ranges.is_empty()
            })
        }
    }

    #[inline]
    fn on_buf(&self) -> impl Fn(api::opts::OnBufArgs) + 'static {
        let inner = self.inner.clone();

        move |(_, buf)| {
            let buf_id = BufferId::from(buf);
            inner.with_mut(|inner| {
                // Draw the highlight ranges for the given buffer.
                if let Some(ranges) = inner.highlight_ranges.get_mut(&buf_id) {
                    ranges.redraw(inner.namespace_id);
                }
            })
        }
    }
}

impl HighlightRanges {
    fn redraw(&mut self, namespace_id: u32) {
        for range in self.inner.values() {
            let opts = api::opts::SetExtmarkOpts::builder()
                .end_row(range.point_range.end.line_idx)
                .end_col(range.point_range.end.byte_offset.into())
                .hl_group(&*range.highlight_group_name)
                .id(range.extmark_id)
                .ephemeral(true)
                .build();

            api::Buffer::from(self.buffer_id)
                .set_extmark(
                    namespace_id,
                    range.point_range.start.line_idx,
                    range.point_range.start.byte_offset.into(),
                    &opts,
                )
                .expect("couldn't set extmark");
        }
    }
}

impl Drop for HighlightRange {
    #[inline]
    fn drop(&mut self) {
        self.decoration_provider.inner.with_mut(|inner| {
            let highlight_ranges = &mut inner
                .highlight_ranges
                .get_mut(&self.buffer_id)
                .expect("not removed until all ranges buffer are dropped")
                .inner;

            highlight_ranges.remove(self.range_id);

            if highlight_ranges.is_empty() {
                inner.highlight_ranges.remove(&self.buffer_id);
            }
        });
    }
}
