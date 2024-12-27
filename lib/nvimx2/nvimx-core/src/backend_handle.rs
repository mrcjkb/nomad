use core::ops::{Deref, DerefMut};

use crate::{ Shared};

/// TODO: docs.
pub(crate) struct BackendHandle<B> {
    inner: Shared<B>,
}

/// TODO: docs.
pub(crate) struct BackendMut<'a, B> {
    backend: &'a mut B,
    handle: BackendHandle<B>,
}

impl<B> BackendHandle<B> {
    #[track_caller]
    #[inline]
    pub(crate) fn with_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(BackendMut<'_, B>) -> R,
    {
        let handle = self.clone();
        self.inner.with_mut(|backend| f(BackendMut { backend, handle }))
    }
}

impl<B> BackendMut<'_, B> {
    #[inline]
    pub(crate) fn handle(&self) -> BackendHandle<B> {
        self.handle.clone()
    }
}

impl<B> Clone for BackendHandle<B> {
    #[inline]
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<B> Deref for BackendMut<'_, B> {
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.backend
    }
}

impl<B> DerefMut for BackendMut<'_, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.backend
    }
}
