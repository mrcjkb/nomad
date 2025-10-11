use core::time::Duration;

use editor::notify::{self, Emitter};
use editor::{Context, Editor};
use flume::TrySendError;
use futures_util::{FutureExt, StreamExt, select_biased};
use neovim::Neovim;

use crate::progress::{JoinState, ProgressReporter, StartState};

/// Frames for the spinner animation.
const SPINNER_FRAMES: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// How many revolutions per minute the spinner should complete.
const SPINNER_RPM: u64 = 75;

/// How often the spinner should be updated in order to achieve the desired
/// RPM.
const SPINNER_UPDATE_INTERVAL: Duration = Duration::from_millis({
    (60_000.0 / ((SPINNER_RPM * SPINNER_FRAMES.len() as u64) as f32)).round()
        as u64
});

pub struct NeovimProgressReporter {
    message_tx: flume::Sender<Message>,
}

struct Message {
    level: notify::Level,
    text: String,
    is_last: bool,
}

impl ProgressReporter<Neovim> for NeovimProgressReporter {
    fn new(ctx: &mut Context<Neovim>) -> Self {
        let (message_tx, message_rx) = flume::bounded::<Message>(4);

        ctx.spawn_local(async move |ctx| {
            let namespace = ctx.namespace().clone();
            let mut spin = async_io::Timer::interval(SPINNER_UPDATE_INTERVAL);
            let mut messages = message_rx.into_stream();

            let Some(mut message) = messages.next().await else { return };
            let mut spinner_frame_idx = 0;
            let mut prev_id = None;

            loop {
                prev_id = ctx.with_editor(|nvim| {
                    Some(nvim.emitter().emit(notify::Notification {
                        level: message.level,
                        message: notify::Message::from_display(format_args!(
                            "{} {}",
                            SPINNER_FRAMES[spinner_frame_idx], message.text,
                        )),
                        namespace: &namespace,
                        updates_prev: prev_id,
                    }))
                });

                if message.is_last {
                    break;
                }

                select_biased! {
                    _ = spin.next().fuse() => {
                        spinner_frame_idx += 1;
                        spinner_frame_idx %= SPINNER_FRAMES.len();
                    },
                    next_message = messages.select_next_some() => {
                        message = next_message
                    },
                }
            }
        })
        .detach();

        Self { message_tx }
    }

    fn report_join_progress(
        &mut self,
        _state: JoinState<'_>,
        _ctx: &mut Context<Neovim>,
    ) {
    }

    fn report_start_progress(
        &mut self,
        state: StartState<'_>,
        _: &mut Context<Neovim>,
    ) {
        let text = match state {
            StartState::ConnectingToServer { .. }
            | StartState::StartingSession => "Connecting to server",
            StartState::ReadingProject { .. } => "Reading project",
            StartState::Done => "Started session",
        };

        let message = Message {
            level: notify::Level::Info,
            text: text.to_owned(),
            is_last: matches!(state, StartState::Done),
        };

        if let Err(err) = self.message_tx.try_send(message) {
            match err {
                TrySendError::Disconnected(_) => unreachable!(),
                TrySendError::Full(_) => {},
            }
        }
    }
}
