use core::ops::Deref;
use std::rc::Rc;

use serde::de::{Deserialize, Deserializer};

#[derive(Clone)]
pub(crate) struct ServerSocket {
    inner: Rc<str>,
}

impl Default for ServerSocket {
    #[inline]
    fn default() -> Self {
        Self { inner: "collab.nomad.foo:64420".to_owned().into() }
    }
}

impl Deref for ServerSocket {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'de> Deserialize<'de> for ServerSocket {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = String::deserialize(deserializer)?;
        Ok(Self { inner: inner.into() })
    }
}
