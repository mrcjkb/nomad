/// TODO: docs.
pub trait EditorCtx {
    /// TODO: docs.
    type Buffer;

    /// TODO: docs.
    fn focused_buffer(&self) -> Self::Buffer;
}
