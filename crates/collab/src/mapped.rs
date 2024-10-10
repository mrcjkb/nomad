use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;

use futures_util::Stream;

pub(crate) struct Mapped<K, V> {
    map: HashMap<K, V>,
}

impl<K, V: Stream> Stream for Mapped<K, V> {
    type Item = V::Item;

    fn poll_next(
        self: Pin<&mut Self>,
        _ctx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

impl<K, V> Deref for Mapped<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K, V> DerefMut for Mapped<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<K, V> Default for Mapped<K, V> {
    fn default() -> Self {
        Self { map: HashMap::new() }
    }
}
