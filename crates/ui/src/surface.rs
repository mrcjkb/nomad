use core::ops::{Range, RangeBounds};

use api::types::*;
use nvim::api;

use crate::{Bound, Cells, HighlightGroup, Point};

pub(crate) type ByteOffset = usize;

/// TODO: docs
pub(crate) struct Surface {
    /// TODO: docs.
    buffer: api::Buffer,

    /// TODO: docs.
    window: api::Window,

    /// TODO: docs.
    namespace: u32,
}

impl Surface {
    /// TODO: docs
    #[inline]
    pub(crate) fn hide(&mut self) {
        let config = WindowConfig::builder().hide(true).build();
        let _ = self.window.set_config(&config);
    }

    /// TODO: docs
    #[inline]
    fn highlight_line_range<R>(
        &mut self,
        line: usize,
        range: R,
        hl: &HighlightGroup,
    ) where
        R: RangeBounds<ByteOffset>,
    {
        hl.set(self.namespace);

        let _ = self.buffer.add_highlight(
            self.namespace,
            hl.name().as_str(),
            line,
            range,
        );
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn highlight_text(
        &mut self,
        range: Range<Point<ByteOffset>>,
        hl: &HighlightGroup,
    ) {
        let start = range.start;

        let end = range.end;

        if start.y() == end.y() {
            self.highlight_line_range(start.y(), start.x()..end.x(), hl);
            return;
        }

        let mut line_range = start.y()..=end.y();

        let Some(first_line) = line_range.next() else { return };

        self.highlight_line_range(first_line, start.x().., hl);

        let Some(last_line) = line_range.next_back() else { return };

        self.highlight_line_range(last_line, ..end.x(), hl);

        for line in line_range {
            self.highlight_line_range(line, .., hl);
        }
    }

    #[inline]
    pub(crate) fn is_hidden(&self) -> bool {
        self.window
            .get_config()
            .map(|config| config.hide.unwrap_or(false))
            .unwrap_or(false)
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new_hidden() -> Self {
        let buffer = api::create_buf(false, true).expect("never fails(?)");

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .height(1)
            .width(1)
            .row(0)
            .col(0)
            .hide(true)
            .style(WindowStyle::Minimal)
            .build();

        let window = api::open_win(&buffer, false, &config)
            .expect("the config is valid");

        let namespace = api::create_namespace("nomad-ui");

        Self { buffer, window, namespace }
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn replace_lines(
        &mut self,
        line_range: impl RangeBounds<usize>,
        replacement: impl Iterator<Item = impl Into<nvim::String>>,
    ) {
        let _ = self.buffer.set_lines(line_range, true, replacement);
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn replace_text(
        &mut self,
        range: Range<Point<ByteOffset>>,
        text: &str,
    ) {
        let lines = text.lines().chain(text.ends_with('\n').then_some(""));

        let _ = self.buffer.set_text(
            range.start.y()..range.end.y(),
            range.start.x(),
            range.end.x(),
            lines,
        );
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn resize_window(&mut self, new_size: Bound<Cells>) {
        let config = WindowConfig::builder()
            .height(new_size.height().into())
            .width(new_size.width().into())
            .build();

        let _ = self.window.set_config(&config);
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn show(&mut self) {
        let config = WindowConfig::builder().hide(false).build();
        let _ = self.window.set_config(&config);
    }
}
