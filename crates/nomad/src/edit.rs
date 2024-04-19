/// TODO: docs
pub trait Edit<Buffer> {
    /// TODO: docs
    type Diff;

    /// TODO: docs
    fn apply(self, buffer: &mut Buffer) -> Self::Diff;
}
