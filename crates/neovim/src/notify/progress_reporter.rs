use editor::context::BorrowState;
use editor::{Context, Editor};
use executor::Executor;

use crate::{Neovim, notify};

/// A percentage value between `0` and `100`.
pub type Percentage = u8;

/// TODO: docs.
pub enum ProgressReporter {
    /// TODO: docs.
    NvimEcho(notify::NvimEchoProgressReporter),

    /// TODO: docs.
    NvimNotify(notify::NvimNotifyProgressReporter),
}

pub(super) struct ProgressNotification {
    pub(super) chunks: notify::Chunks,
    pub(super) kind: ProgressNotificationKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum ProgressNotificationKind {
    Progress(Option<notify::Percentage>),
    Success,
    Error,
}

impl ProgressReporter {
    /// Creates a new progress reporter.
    pub fn new(ctx: &mut Context<Neovim, impl BorrowState>) -> Self {
        let namespace = ctx.namespace().clone();

        ctx.with_editor(|nvim| {
            let local_spawner = nvim.executor().local_spawner();

            if notify::NvimNotify::is_installed() {
                Self::NvimNotify(notify::NvimNotifyProgressReporter::new(
                    namespace,
                    local_spawner,
                ))
            } else {
                Self::NvimEcho(notify::NvimEchoProgressReporter::new(
                    namespace,
                    local_spawner,
                ))
            }
        })
    }

    /// TODO: docs.
    pub fn report_error(self, chunks: notify::Chunks) {
        let notif = ProgressNotification {
            chunks,
            kind: ProgressNotificationKind::Error,
        };
        match self {
            Self::NvimEcho(inner) => inner.send_notification(notif),
            Self::NvimNotify(inner) => inner.send_notification(notif),
        }
    }

    /// TODO: docs.
    pub fn report_progress(
        &self,
        chunks: notify::Chunks,
        percentage: Option<Percentage>,
    ) {
        let notif = ProgressNotification {
            chunks,
            kind: ProgressNotificationKind::Progress(percentage),
        };
        match self {
            Self::NvimEcho(inner) => inner.send_notification(notif),
            Self::NvimNotify(inner) => inner.send_notification(notif),
        }
    }

    /// TODO: docs.
    pub fn report_success(self, chunks: notify::Chunks) {
        let notif = ProgressNotification {
            chunks,
            kind: ProgressNotificationKind::Success,
        };
        match self {
            Self::NvimEcho(inner) => inner.send_notification(notif),
            Self::NvimNotify(inner) => inner.send_notification(notif),
        }
    }
}

impl From<ProgressNotificationKind> for notify::Level {
    fn from(kind: ProgressNotificationKind) -> Self {
        use ProgressNotificationKind::*;
        match kind {
            Progress(_) | Success => Self::Info,
            Error => Self::Error,
        }
    }
}
