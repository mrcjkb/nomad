use crate::{Cells, RequestedBound, SceneFragment};

/// TODO: docs
pub trait Render {
    /// TODO: docs
    fn layout(&self) -> RequestedBound<Cells>;

    /// TODO: docs
    fn paint(&self, scene_fragment: &mut SceneFragment);
}
