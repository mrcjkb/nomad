/// TODO: docs.
pub struct SceneFragment {}

impl SceneFragment {
    /// TODO: docs
    #[inline]
    pub fn cutout<C: Cutout>(&mut self, cutout: C) -> C::Cutout {
        cutout.cutout(self)
    }

    /// TODO: docs
    #[inline]
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

/// TODO: docs.
pub trait Cutout {
    /// TODO: docs.
    type Cutout;

    /// TODO: docs.
    fn cutout(self, framgnent: &mut SceneFragment) -> Self::Cutout;
}
