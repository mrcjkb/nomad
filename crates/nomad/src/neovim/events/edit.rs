use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::Range;

use nvim_oxi::api;

use crate::neovim::{BufferId, Neovim, Offset, Point};
use crate::{ActorId, ByteOffset, Context, Emitter, Event, Shared, Text};

/// TODO: docs.
#[derive(Clone)]
pub struct Edit<T> {
    actor_id: ActorId,
    hunk: Hunk<T>,
}

/// TODO: docs.
pub struct EditEvent<T> {
    id: BufferId,
    next_edit_made_by: Shared<Option<ActorId>>,
    ty: PhantomData<T>,
}

#[derive(Clone)]
struct Hunk<T> {
    deleted_range: Range<T>,
    inserted_text: Text,
}

impl<T> Edit<T> {
    /// TODO: docs.
    pub fn actor_id(&self) -> ActorId {
        self.actor_id
    }

    /// TODO: docs.
    pub fn deleted_range(&self) -> &Range<T> {
        &self.hunk.deleted_range
    }

    /// TODO: docs.
    pub fn inserted_text(&self) -> &Text {
        &self.hunk.inserted_text
    }
}

impl<T> EditEvent<T> {
    pub(crate) fn new(
        id: BufferId,
        next_edit_made_by: Shared<Option<ActorId>>,
    ) -> Self {
        Self { id, next_edit_made_by, ty: PhantomData }
    }
}

impl<T: Offset + Clone> Event<Neovim> for EditEvent<T> {
    type Payload = Edit<T>;
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
                    let edit = Edit { actor_id, hunk: Hunk::from(args) };
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

impl<T: Offset> From<api::opts::OnBytesArgs> for Hunk<T> {
    #[inline]
    fn from(args: nvim_oxi::api::opts::OnBytesArgs) -> Self {
        let &(
            ref _bytes,
            ref buf,
            _changedtick,
            start_row,
            start_col,
            start_offset,
            _old_end_row,
            _old_end_col,
            _old_end_len,
            new_end_row,
            new_end_col,
            _new_end_len,
        ) = &args;

        let buf = BufferId::new(buf.clone());

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

        let inserted_text = if start == end {
            Text::new()
        } else {
            buf.get_text_in_point_range(start..end)
        };

        let deleted_range = T::deleted_range(&args);

        Hunk { deleted_range, inserted_text }
    }
}

impl<T> PartialEq for EditEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for EditEvent<T> {}

impl<T> PartialOrd for EditEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for EditEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
