use super::{Level, Message, NotificationId, Source};

/// TODO: docs.
pub struct Notification {
    /// TODO: docs.
    pub level: Level,

    /// TODO: docs.
    pub source: Source,

    /// TODO: docs.
    pub message: Message,

    /// TODO: docs.
    pub updates_prev: Option<NotificationId>,
}
