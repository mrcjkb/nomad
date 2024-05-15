use compact_str::CompactString;

use crate::{Bound, Cells, SceneFragment, Surface};

/// TODO: docs
pub(crate) struct Scene {
    lines: Vec<SceneLine>,
}

impl Scene {
    /// Turns the entire `Scene` into a `SceneFragment` which can be used in
    /// the [`paint`](crate::Render::paint) method of a
    /// [`Render`](crate::Render) implementation.
    #[inline]
    pub(crate) fn as_fragment(&mut self) -> SceneFragment<'_> {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn diff(&self) -> SceneDiff<'_> {
        todo!();
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new() -> Self {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn resize(&mut self, new_size: Bound<Cells>) {
        todo!();
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn size(&self) -> Bound<Cells> {
        todo!();
    }
}

/// TODO: docs
struct SceneLine {
    runs: Vec<SceneRun>,
}

/// TODO: docs
struct SceneRun {
    /// TODO: docs.
    text: CompactString,
}

/// TODO: docs
pub(crate) struct SceneDiff<'a> {
    fragment: SceneFragment<'a>,
}

impl<'a> SceneDiff<'a> {
    /// TODO: docs
    #[inline]
    pub(crate) fn apply(self, _surface: &mut Surface) {
        todo!()
    }
}
