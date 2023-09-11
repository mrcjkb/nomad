use std::cmp::Ordering;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use common::nvim::{self, api::opts::*};

use crate::*;

pub type ViewId = nvim::api::Window;

/// TODO: docs.
pub(crate) struct View {
    /// TODO: docs.
    buffer: nvim::api::Buffer,

    /// TODO: docs.
    id: ViewId,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the path has no parent directory")]
    NoParentDir,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct File {
    path: PathBuf,
    file_name: OsString,
}

fn siblings(of_path: &Path) -> Result<Vec<File>, Error> {
    let parent = of_path.parent().ok_or(Error::NoParentDir)?;
    let siblings = std::fs::read_dir(parent)?;
    let mut files = Vec::new();
    for file in siblings {
        let file = file?;
        let path = file.path();
        let file_name = file.file_name();
        files.push(File { file_name, path });
    }
    Ok(files)
}

fn sort_first_by_directory_then_by_name(a: &File, b: &File) -> Ordering {
    match (a.path.is_dir(), b.path.is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.path.cmp(&b.path),
    }
}

impl View {
    /// TODO: docs.
    pub fn close(self) {
        self.id.close(true).unwrap();

        self.buffer
            .delete(&BufDeleteOpts::builder().force(true).build())
            .unwrap();
    }

    /// TODO: docs.
    pub fn id(&self) -> ViewId {
        self.id.clone()
    }

    /// TODO: docs.
    pub fn new(
        at_path: PathBuf,
        with_config: &WindowConfig,
    ) -> Result<Self, Error> {
        let siblings = {
            let mut s = siblings(&at_path)?;
            s.sort_by(sort_first_by_directory_then_by_name);
            s
        };

        let mut buffer = nvim::api::create_buf(false, true).unwrap();

        buffer
            .set_lines(
                0..0,
                true,
                siblings
                    .iter()
                    .map(|file| file.file_name.to_string_lossy().into_owned())
                    .map(|path| nvim::String::from(path.as_str())),
            )
            .unwrap();

        let window = nvim::api::open_win(&buffer, true, &(with_config.into()))
            .expect("the config is valid");

        Ok(Self { buffer, id: window })
    }
}
