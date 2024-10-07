use core::cmp::Ordering;

use nvim_oxi::api;

use crate::neovim::{BufferId, Neovim, Point};
use crate::{
    ActorId,
    ByteOffset,
    Context,
    Edit,
    Emitter,
    Event,
    Hunk,
    Shared,
    Text,
};

/// TODO: docs.
pub struct EditEvent {
    pub(in crate::neovim) id: BufferId,
    pub(in crate::neovim) next_edit_made_by: Shared<Option<ActorId>>,
}

impl Event<Neovim> for EditEvent {
    type Payload = Edit;
    type SubscribeCtx = Shared<bool>;

    fn subscribe(
        &mut self,
        emitter: Emitter<Self::Payload>,
        _: &Context<Neovim>,
    ) -> Self::SubscribeCtx {
        let should_detach = Shared::new(false);

        let opts = api::opts::BufAttachOpts::builder()
            .on_bytes({
                let next_edit_made_by = self.next_edit_made_by.clone();
                let should_detach = should_detach.clone();
                move |args| {
                    let actor_id = next_edit_made_by
                        .with_mut(Option::take)
                        .unwrap_or(ActorId::unknown());
                    let edit = Edit::new(actor_id, [Hunk::from(args)]);
                    emitter.send(edit);
                    should_detach.get()
                }
            })
            .build();

        if let Err(err) = self.id.as_nvim().attach(false, &opts) {
            panic!("couldn't attach to {:?}: {err}", self.id);
        }

        should_detach
    }

    fn unsubscribe(
        &mut self,
        should_detach: Self::SubscribeCtx,
        _: &Context<Neovim>,
    ) {
        should_detach.set(true);
    }
}

impl From<api::opts::OnBytesArgs> for Hunk {
    #[inline]
    fn from(
        (
            _bytes,
            buf,
            _changedtick,
            start_row,
            start_col,
            start_offset,
            _old_end_row,
            _old_end_col,
            old_end_len,
            new_end_row,
            new_end_col,
            _new_end_len,
        ): nvim_oxi::api::opts::OnBytesArgs,
    ) -> Self {
        let buf = BufferId::new(buf);

        let start = Point {
            line_idx: start_row,
            byte_offset: ByteOffset::new(start_offset),
        };

        let end = Point {
            line_idx: start_row + new_end_row,
            byte_offset: (start_col * (new_end_row == 0) as usize
                + new_end_col)
                .into(),
        };

        let replacement = if start == end {
            Text::new()
        } else {
            buf.get_text_in_point_range(start..end)
        };

        let deleted_range =
            start_offset.into()..(start_offset + old_end_len).into();

        Hunk::new(deleted_range, replacement.as_str())
    }
}

impl PartialEq for EditEvent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for EditEvent {}

impl PartialOrd for EditEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EditEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
