use crate::backend_handle::BackendMut;

/// TODO: docs.
pub struct NeovimCtx<'a, B> {
    backend: BackendMut<'a, B>,
}

impl<'a, B> NeovimCtx<'a, B> {
    #[inline]
    pub(crate) fn new(handle: BackendMut<'a, B>) -> Self {
        Self { backend: handle }
    }
}
