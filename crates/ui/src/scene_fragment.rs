use crate::Cells;

/// TODO: docs.
pub struct SceneFragment {}

impl SceneFragment {
    /// TODO: docs
    #[inline]
    pub fn cutout<C: Cutout>(
        &mut self,
        cutout: C,
    ) -> (&mut Self, C::Cutout<'_>) {
        cutout.cutout(self)
    }

    /// TODO: docs
    #[inline]
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub fn height(&self) -> Cells {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub fn split_x(&mut self, _split_at: Cells) -> (&mut Self, &mut Self) {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub fn split_y(&mut self, _split_at: Cells) -> (&mut Self, &mut Self) {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub fn width(&self) -> Cells {
        todo!()
    }
}

/// TODO: docs.
pub trait Cutout {
    /// TODO: docs.
    type Cutout<'a>;

    /// TODO: docs.
    fn cutout(
        self,
        fragment: &mut SceneFragment,
    ) -> (&mut SceneFragment, Self::Cutout<'_>);
}
