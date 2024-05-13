use crate::{Cells, ExpandRect, Render, RequestedBound, SceneFragment};

/// TODO: docs
pub struct Margin<R> {
    inner: R,
    expand: ExpandRect<Cells>,
}

impl<R> Margin<R> {
    #[inline]
    pub(crate) fn new(inner: R, expand: ExpandRect<Cells>) -> Self {
        Self { inner, expand }
    }
}

impl<R: Render> Render for Margin<R> {
    #[inline]
    fn layout(&self) -> RequestedBound<Cells> {
        self.inner.layout().map(|bound| bound + self.expand)
    }

    #[inline]
    fn paint(&self, fragment: &mut SceneFragment) {
        let _ = fragment.cutout(self.expand);

        if !fragment.is_empty() {
            self.inner.paint(fragment);
        }
    }
}
