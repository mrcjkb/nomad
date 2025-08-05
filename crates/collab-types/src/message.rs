use crate::{Peer, PeerId, ProjectRequest, ProjectResponse, binary, fs, text};

/// TODO: docs.
#[derive(
    cauchy::Debug,
    Clone,
    cauchy::PartialEq,
    cauchy::Eq,
    cauchy::From,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Message {
    /// TODO: docs.
    CreatedCursor(#[from] text::CursorCreation),

    /// TODO: docs.
    CreatedDirectory(#[from] fs::DirectoryCreation),

    /// TODO: docs.
    CreatedFile(#[from] fs::FileCreation),

    /// TODO: docs.
    CreatedSelection(#[from] text::SelectionCreation),

    /// TODO: docs.
    DeletedDirectory(#[from] fs::DirectoryDeletion),

    /// TODO: docs.
    DeletedFile(#[from] fs::FileDeletion),

    /// TODO: docs.
    EditedBinary(#[from] binary::BinaryEdit),

    /// TODO: docs.
    EditedText(#[from] text::TextEdit),

    /// TODO: docs.
    MovedCursor(#[from] text::CursorMove),

    /// TODO: docs.
    MovedDirectory(#[from] fs::DirectoryMove),

    /// TODO: docs.
    MovedFile(#[from] fs::FileMove),

    /// TODO: docs.
    MovedSelection(#[from] text::SelectionMove),

    /// TODO: docs.
    PeerDisconnected(PeerId),

    /// TODO: docs.
    PeerJoined(Peer),

    /// TODO: docs.
    PeerLeft(PeerId),

    /// TODO: docs.
    ProjectRequest(#[from] ProjectRequest),

    /// TODO: docs.
    ProjectResponse(#[from] ProjectResponse),

    /// TODO: docs.
    RemovedCursor(text::CursorRemoval),

    /// TODO: docs.
    RemovedSelection(text::SelectionRemoval),

    /// TODO: docs.
    RenamedFsNode(#[from] fs::Rename),

    /// TODO: docs.
    SavedTextFile(fs::GlobalFileId),
}
