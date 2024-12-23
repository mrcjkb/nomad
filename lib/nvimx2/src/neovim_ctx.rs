use crate::Shared;

/// TODO: docs.
pub struct NeovimCtx<'a, B> {
    _backend_mut: &'a mut B,
    _backend_instance: Shared<B>,
}
