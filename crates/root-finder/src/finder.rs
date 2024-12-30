use futures_util::{StreamExt, pin_mut};
use nvimx::fs::{self, AbsPath, AbsPathBuf, DirEntry, FsNodeKind};

use crate::{FindRootError, Marker};

/// TODO: docs.
pub struct Finder<Fs> {
    fs: Fs,
}

impl<Fs: fs::Fs> Finder<Fs> {
    /// TODO: docs.
    pub async fn find_root<P, M>(
        &self,
        start_from: P,
        marker: M,
    ) -> Result<Option<AbsPathBuf>, FindRootError<Fs>>
    where
        P: AsRef<AbsPath>,
        M: Marker,
    {
        let node_kind = self
            .fs
            .node_at_path(start_from.as_ref())
            .await
            .map_err(FindRootError::NodeAtStartPath)?
            .ok_or(FindRootError::StartPathNotFound)?
            .kind();

        let mut dir = match node_kind {
            FsNodeKind::Directory => start_from.as_ref(),
            FsNodeKind::File => start_from
                .as_ref()
                .parent()
                .expect("path is of file, so it must have a parent"),
            FsNodeKind::Symlink => todo!("can't handle symlinks yet"),
        }
        .to_owned();

        loop {
            if contains_marker(&dir, &marker, &self.fs).await? {
                return Ok(Some(dir));
            }
            if !dir.pop() {
                return Ok(None);
            }
        }
    }

    /// TODO: docs.
    pub fn new(fs: Fs) -> Self {
        Self { fs }
    }
}

async fn contains_marker<Fs: fs::Fs>(
    dir_path: &AbsPath,
    marker: &impl Marker,
    fs: &Fs,
) -> Result<bool, FindRootError<Fs>> {
    let entries = fs.read_dir(dir_path).await.map_err(|err| {
        FindRootError::ReadDir { dir_path: dir_path.to_owned(), err }
    })?;
    pin_mut!(entries);

    while let Some(res) = entries.next().await {
        let entry = res.map_err(|err| FindRootError::DirEntry {
            parent_path: dir_path.to_owned(),
            err,
        })?;

        let fs_node_name =
            entry.name().await.map_err(|err| FindRootError::DirEntryName {
                parent_path: dir_path.to_owned(),
                err,
            })?;

        let fs_node_kind = entry.node_kind().await.map_err(|err| {
            let mut entry_path = dir_path.to_owned();
            entry_path.push(&*fs_node_name);
            FindRootError::DirEntryNodeKind { entry_path, err }
        })?;

        if marker.matches(&*fs_node_name, fs_node_kind) {
            return Ok(true);
        }
    }

    Ok(false)
}
