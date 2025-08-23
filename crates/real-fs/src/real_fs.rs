//! TODO: docs.

use std::io;
use std::time::SystemTime;

use abs_path::AbsPath;

use crate::{Directory, File, Metadata, Symlink};

/// TODO: docs.
pub type Inode = u64;

/// TODO: docs.
#[derive(Debug, Default, Copy, Clone)]
pub struct RealFs {}

impl fs::Fs for RealFs {
    type Directory = Directory;
    type File = File;
    type Symlink = Symlink;
    type Metadata = Metadata;
    type NodeId = Inode;
    type Timestamp = SystemTime;

    type CreateDirectoriesError = io::Error;
    type NodeAtPathError = io::Error;

    #[inline]
    async fn create_all_missing_directories<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> Result<Self::Directory, Self::CreateDirectoriesError> {
        let path = path.as_ref();
        async_fs::create_dir_all(path).await?;
        let metadata = async_fs::metadata(path).await?;
        Ok(Directory { path: path.to_owned(), metadata })
    }

    #[inline]
    async fn node_at_path<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> Result<Option<fs::Node<Self>>, Self::NodeAtPathError> {
        let path = path.as_ref();
        let metadata = match async_fs::symlink_metadata(path).await {
            Ok(metadata) => metadata,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        let Ok(file_type) = metadata.file_type().try_into() else {
            return Ok(None);
        };
        Ok(Some(match file_type {
            fs::NodeKind::File => fs::Node::File(File {
                file: None,
                metadata,
                path: path.to_owned(),
            }),
            fs::NodeKind::Directory => fs::Node::Directory(Directory {
                metadata,
                path: path.to_owned(),
            }),
            fs::NodeKind::Symlink => {
                fs::Node::Symlink(Symlink { metadata, path: path.to_owned() })
            },
        }))
    }

    #[inline]
    fn now(&self) -> Self::Timestamp {
        SystemTime::now()
    }
}
