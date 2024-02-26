use crate::SetCtx;

/// TODO: docs
pub struct Set<T> {
    inner: pond::Set<T>,
}

impl<T> Clone for Set<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T> Set<T> {
    #[inline]
    pub(crate) fn new(inner: pond::Set<T>) -> Self {
        Self { inner }
    }

    /// TODO: docs
    #[inline]
    pub fn update<F>(&self, _update_with: F, _ctx: &mut SetCtx)
    where
        F: FnOnce(&mut T),
    {
        todo!();
    }
}
