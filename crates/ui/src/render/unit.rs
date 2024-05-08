use crate::{Cells, Render, RequestedBound, SceneFragment};

impl Render for () {
    #[inline]
    fn layout(&self) -> RequestedBound<Cells> {
        todo!()
    }

    #[inline]
    fn paint(&self, _: &mut SceneFragment) {
        todo!()
    }
}
