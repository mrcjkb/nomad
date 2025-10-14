use editor::notify::{Emitter, Notification, NotificationId};

use crate::convert::Convert;
use crate::executor::NeovimLocalSpawner;
use crate::notify::{NvimEcho, NvimNotify};

/// TODO: docs.
pub struct NeovimEmitter<'ex> {
    local_spawner: &'ex mut NeovimLocalSpawner,
}

impl<'ex> NeovimEmitter<'ex> {
    pub(crate) fn new(local_spawner: &'ex mut NeovimLocalSpawner) -> Self {
        Self { local_spawner }
    }
}

impl Emitter for NeovimEmitter<'_> {
    fn emit(&mut self, notification: Notification) -> NotificationId {
        let namespace = notification.namespace;
        let chunks = notification.message.into();
        let level = notification.level.convert();
        if NvimNotify::is_installed() {
            NvimNotify::notify(namespace, chunks, level);
        } else {
            NvimEcho::notify(namespace, chunks, level, self.local_spawner);
        }
        NotificationId::new(0)
    }
}
