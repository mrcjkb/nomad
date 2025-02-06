use crate::{ByteOffset, fs};

/// TODO: docs.
pub struct Metadata<Fs: fs::Fs> {
    /// TODO: docs.
    pub created_at: Option<Fs::Timestamp>,

    /// TODO: docs.
    pub last_modified_at: Option<Fs::Timestamp>,

    /// TODO: docs.
    pub len: ByteOffset,
}

#[cfg(feature = "os-fs")]
impl From<async_fs::Metadata> for Metadata<fs::os::OsFs> {
    #[inline]
    fn from(metadata: async_fs::Metadata) -> Self {
        Self {
            created_at: metadata.created().ok(),
            last_modified_at: metadata.modified().ok(),
            len: metadata.len().into(),
        }
    }
}
