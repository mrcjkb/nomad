use core::pin::Pin;

use futures_util::stream::{self, Stream, StreamExt};
use futures_util::{FutureExt, pin_mut, select};
use nvimx2::fs;

use crate::dir_entry::DirEntry;
use crate::filter::{Filter, Filtered};

/// TODO: docs.
pub trait WalkDir: Sized {
    /// TODO: docs.
    type DirEntry: fs::DirEntry;

    /// TODO: docs.
    type DirEntryError;

    /// TODO: docs.
    type ReadDirError;

    /// TODO: docs.
    fn read_dir(
        &self,
        dir_path: &fs::AbsPath,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = Result<Self::DirEntry, Self::DirEntryError>>,
            Self::ReadDirError,
        >,
    >;

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter<Self>,
    {
        Filtered::new(filter, self)
    }

    /// TODO: docs.
    #[inline]
    fn for_each<'a, H>(
        &'a self,
        dir_path: fs::AbsPathBuf,
        handler: H,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalkError<Self>>> + 'a>>
    where
        H: AsyncFn(&fs::AbsPath, DirEntry<Self>) + Clone + 'a,
    {
        Box::pin(async move {
            let entries = match self.read_dir(&dir_path).await {
                Ok(entries) => entries.fuse(),
                Err(err) => {
                    return Err(WalkError {
                        dir_path: dir_path.clone(),
                        kind: WalkErrorKind::ReadDir(err),
                    });
                },
            };
            let mut create_entries = stream::FuturesUnordered::new();
            let mut handle_entries = stream::FuturesUnordered::new();
            let mut read_children = stream::FuturesUnordered::new();
            pin_mut!(entries);
            loop {
                select! {
                    res = entries.select_next_some() => {
                        let entry = res.map_err(|err| WalkError {
                            dir_path: dir_path.clone(),
                            kind: WalkErrorKind::DirEntry(err),
                        })?;
                        create_entries.push(DirEntry::new(entry));
                    },
                    res = create_entries.select_next_some() => {
                        let entry = res.map_err(|kind| WalkError {
                            dir_path: dir_path.clone(),
                            kind,
                        })?;
                        if entry.node_kind().is_dir() {
                            let mut dir_path = dir_path.clone();
                            dir_path.push(entry.name());
                            let handler = handler.clone();
                            read_children.push(self.for_each(dir_path, handler));
                        }
                        handle_entries.push(handler(&dir_path, entry));
                    },
                    () = handle_entries.select_next_some() => (),
                    res = read_children.select_next_some() => res?,
                }
            }
        })
    }

    /// TODO: docs.
    #[inline]
    fn paths(
        &self,
        dir_path: fs::AbsPathBuf,
    ) -> impl Stream<Item = Result<fs::AbsPathBuf, WalkError<Self>>> {
        self.to_stream(dir_path, async |parent_path, entry| {
            let mut path = parent_path.to_owned();
            path.push(entry.name());
            path
        })
    }

    /// TODO: docs.
    #[inline]
    fn to_stream<'a, H, T>(
        &'a self,
        dir_path: fs::AbsPathBuf,
        handler: H,
    ) -> impl Stream<Item = Result<T, WalkError<Self>>> + 'a
    where
        H: AsyncFn(&fs::AbsPath, DirEntry<Self>) -> T + Clone + 'a,
        T: 'a,
    {
        let (tx, rx) = flume::unbounded();
        let for_each = self
            .for_each(dir_path, async move |path, entry| {
                let payload = handler(path, entry).await;
                let _ = tx.send(payload);
            })
            .boxed_local()
            .fuse();
        futures_util::stream::unfold(
            (for_each, rx),
            move |(mut for_each, rx)| async move {
                let res = select! {
                    res = for_each => match res {
                        Ok(()) => return None,
                        Err(err) => Err(err),
                    },
                    res = rx.recv_async() => match res {
                        Ok(payload) => Ok(payload),
                        Err(_err) => return None,
                    },
                };
                Some((res, (for_each, rx)))
            },
        )
    }
}

/// TODO: docs.
pub struct WalkError<W: WalkDir> {
    /// TODO: docs.
    pub dir_path: fs::AbsPathBuf,

    /// TODO: docs.
    pub kind: WalkErrorKind<W>,
}

/// TODO: docs.
pub enum WalkErrorKind<W: WalkDir> {
    /// TODO: docs.
    DirEntry(W::DirEntryError),

    /// TODO: docs.
    DirEntryName(<W::DirEntry as fs::DirEntry>::NameError),

    /// TODO: docs.
    DirEntryNodeKind(<W::DirEntry as fs::DirEntry>::NodeKindError),

    /// TODO: docs.
    ReadDir(W::ReadDirError),
}

impl<Fs: fs::Fs> WalkDir for Fs {
    type DirEntry = <Self as fs::Fs>::DirEntry;
    type DirEntryError = <Self as fs::Fs>::DirEntryError;
    type ReadDirError = <Self as fs::Fs>::ReadDirError;

    async fn read_dir(
        &self,
        dir_path: &fs::AbsPath,
    ) -> Result<<Self as fs::Fs>::ReadDir, Self::ReadDirError> {
        fs::Fs::read_dir(self, dir_path).await
    }
}
