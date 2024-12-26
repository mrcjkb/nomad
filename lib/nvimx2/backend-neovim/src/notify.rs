//! TODO: docs.

use core::iter;

use nvimx_core::notify::{Emitter, Level, Notification, NotificationId};
use smallvec::SmallVec;

use crate::convert::Convert;
use crate::oxi;

/// TODO: docs.
pub trait VimNotifyProvider: 'static {
    /// TODO: docs.
    fn to_message(&mut self, notification: &Notification) -> String;

    /// TODO: docs.
    fn to_opts(&mut self, notification: &Notification) -> oxi::Dictionary;

    /// TODO: docs.
    fn to_notification_id(
        &mut self,
        notify_return: oxi::Object,
    ) -> NotificationId;
}

/// TODO: docs.
pub enum NeovimEmitter {
    /// TODO: docs.
    VimNotify(VimNotify),

    /// TODO: docs.
    Custom(Box<dyn Emitter>),
}

/// TODO: docs.
pub struct VimNotify {
    provider: Box<dyn VimNotifyProvider>,
}

struct DefaultProvider;

impl VimNotify {
    /// TODO: docs.
    #[inline]
    pub fn new<P: VimNotifyProvider>(provider: P) -> Self {
        Self { provider: Box::new(provider) }
    }
}

impl Emitter for NeovimEmitter {
    #[inline]
    fn emit(&mut self, notification: Notification) -> NotificationId {
        match self {
            Self::VimNotify(emitter) => emitter.emit(notification),
            Self::Custom(emitter) => emitter.emit(notification),
        }
    }
}

impl Emitter for VimNotify {
    #[inline]
    fn emit(&mut self, notification: Notification) -> NotificationId {
        let message = self.provider.to_message(&notification);
        let level = notification.level.convert();
        let opts = self.provider.to_opts(&notification);
        match oxi::api::notify(&message, level, &opts) {
            Ok(obj) => self.provider.to_notification_id(obj),
            Err(_err) => NotificationId::new(0),
        }
    }
}

impl Default for NeovimEmitter {
    #[inline]
    fn default() -> Self {
        Self::VimNotify(Default::default())
    }
}

impl Default for VimNotify {
    #[inline]
    fn default() -> Self {
        Self::new(DefaultProvider)
    }
}

impl VimNotifyProvider for DefaultProvider {
    #[inline]
    fn to_message(&mut self, notification: &Notification) -> String {
        let src = &notification.source;

        let tag = iter::once(src.plugin_name.as_str())
            .chain(src.module_name.map(|n| n.as_str()))
            .chain(src.action_name.map(|n| n.as_str()))
            .collect::<SmallVec<[_; 3]>>()
            .join(".");

        format!("[{tag}] {}", notification.message)
    }

    #[inline]
    fn to_opts(&mut self, _: &Notification) -> oxi::Dictionary {
        oxi::Dictionary::new()
    }

    #[inline]
    fn to_notification_id(&mut self, _: oxi::Object) -> NotificationId {
        NotificationId::new(0)
    }
}

impl VimNotifyProvider for Box<dyn VimNotifyProvider> {
    #[inline]
    fn to_message(&mut self, notification: &Notification) -> String {
        (**self).to_message(notification)
    }

    #[inline]
    fn to_opts(&mut self, notification: &Notification) -> oxi::Dictionary {
        (**self).to_opts(notification)
    }

    #[inline]
    fn to_notification_id(
        &mut self,
        notify_return: oxi::Object,
    ) -> NotificationId {
        (**self).to_notification_id(notify_return)
    }
}

impl Convert<oxi::api::types::LogLevel> for Level {
    #[inline]
    fn convert(self) -> oxi::api::types::LogLevel {
        match self {
            Self::Trace => oxi::api::types::LogLevel::Trace,
            Self::Debug => oxi::api::types::LogLevel::Debug,
            Self::Info => oxi::api::types::LogLevel::Info,
            Self::Warn => oxi::api::types::LogLevel::Warn,
            Self::Error => oxi::api::types::LogLevel::Error,
        }
    }
}
