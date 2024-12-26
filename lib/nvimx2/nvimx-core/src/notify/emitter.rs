use super::{Notification, NotificationId};

/// TODO: docs.
pub trait Emitter {
    /// TODO: docs.
    fn emit(&mut self, notification: Notification) -> NotificationId;
}
