use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;

use futures_util::Stream;

pub(crate) struct StreamMap<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> Stream for StreamMap<K, V>
where
    K: Unpin,
    V: Unpin + Stream,
{
    type Item = V::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        ctx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        for stream in self.map.values_mut() {
            match Pin::new(stream).poll_next(ctx) {
                Poll::Ready(Some(val)) => return Poll::Ready(Some(val)),
                _ => continue,
            }
        }

        Poll::Pending
    }
}

impl<K, V> Deref for StreamMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K, V> DerefMut for StreamMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<K, V> Default for StreamMap<K, V> {
    fn default() -> Self {
        Self { map: HashMap::new() }
    }
}
