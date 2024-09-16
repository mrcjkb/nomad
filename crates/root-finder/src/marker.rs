use core::future::Future;
use std::io;

use collab_fs::{AbsUtf8Path, Fs};

/// TODO: docs.
pub trait Marker: Sized {
    /// Whether the marker matches the file or directory at the given path.
    fn matches<F: Fs>(
        &self,
        path: &AbsUtf8Path,
        metadata: &F::Metadata,
        fs: &F,
    ) -> impl Future<Output = io::Result<bool>>;

    /// Combines this marker with another marker.
    fn combine<T: Marker>(self, other: T) -> (Self, T) {
        (self, other)
    }
}

impl<M1, M2> Marker for (M1, M2)
where
    M1: Marker,
    M2: Marker,
{
    async fn matches<F: Fs>(
        &self,
        path: &AbsUtf8Path,
        metadata: &F::Metadata,
        fs: &F,
    ) -> io::Result<bool> {
        let (m1, m2) = self;
        let m1_matches = m1.matches(path, metadata, fs).await?;
        let m2_matches = m2.matches(path, metadata, fs).await?;
        Ok(m1_matches && m2_matches)
    }
}
