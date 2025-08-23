//! TODO: docs.

mod directory;
mod file;
mod metadata;
mod real_fs;
mod symlink;
#[cfg(feature = "temp")]
mod temp;

pub use directory::Directory;
pub use file::File;
pub use metadata::Metadata;
pub use real_fs::{Inode, RealFs};
pub use symlink::Symlink;
#[cfg(feature = "temp")]
pub use temp::{TempDirectory, TempFile};

/// Moves the node at the given source path to the target path.
///
/// Note that this will error if (among other things) there's already a node at
/// the target path.
async fn move_node(
    source_path: &abs_path::AbsPath,
    target_path: &abs_path::AbsPath,
) -> Result<(), std::io::Error> {
    // FIXME: this could lead to a TOC-TOU race condition.
    if async_fs::symlink_metadata(target_path).await.is_ok() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "target path already exists",
        ));
    }
    async_fs::rename(source_path, target_path).await
}
