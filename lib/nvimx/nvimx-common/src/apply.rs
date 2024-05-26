/// TODO: docs
pub trait Apply<Edit> {
    /// TODO: docs
    type Diff;

    /// TODO: docs
    fn apply(&mut self, edit: Edit) -> Self::Diff;
}
