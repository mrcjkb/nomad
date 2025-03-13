use nvimx_core::notify::{self, Emitter, NotificationId};

#[derive(Default)]
pub struct TestEmitter {
    notifications: Vec<Notification>,
}

pub struct Notification {
    pub level: notify::Level,
    pub message: notify::Message,
    pub namespace: notify::Namespace,
}

impl Emitter for TestEmitter {
    fn emit(&mut self, notification: notify::Notification) -> NotificationId {
        match notification.updates_prev {
            Some(id) => {
                let idx: usize = id.into_u64().try_into().expect("oob");
                self.notifications[idx] = notification.into();
                id
            },
            None => {
                let idx = self.notifications.len();
                self.notifications.push(notification.into());
                NotificationId::new(idx.try_into().expect("oob"))
            },
        }
    }
}

impl From<notify::Notification<'_>> for Notification {
    fn from(notification: notify::Notification) -> Self {
        Self {
            level: notification.level,
            message: notification.message,
            namespace: notification.namespace.clone(),
        }
    }
}
