use core::future::Future;
use std::io;

use collab_fs::{AbsUtf8Path, Fs};

/// TODO: docs.
pub trait Marker {
    /// Whether the marker matches the file or directory at the given path.
    fn matches<F: Fs>(
        &self,
        path: &AbsUtf8Path,
        metadata: &F::Metadata,
        fs: &F,
    ) -> impl Future<Output = io::Result<bool>>;
}
