use core::cell::Cell;
use core::iter;
use std::rc::Rc;

use editor::{AccessMut, AgentId, Edit, Editor, Replacement, Shared};
use nohash::IntMap as NoHashMap;
use nvim_oxi::api::opts::ShouldDetach;
use smallvec::{SmallVec, smallvec_inline};

use crate::Neovim;
use crate::buffer::{BufferExt, BufferId, NeovimBuffer, Point};
use crate::events::{AutocmdId, Callbacks, Event, EventKind, Events};
use crate::option::{NeovimOption, UneditableEndOfLine};
use crate::oxi::api;
use crate::utils::CallbackExt;

const TRIGGER_AUTOCMD_PATTERN: &str = "BufferEditedEventTrigger";

#[derive(Debug, Copy, Clone)]
pub(crate) struct BufferEdited(pub(crate) BufferId);

impl BufferEdited {
    fn on_bytes(
        nvim: &mut impl AccessMut<Neovim>,
        args: api::opts::OnBytesArgs,
    ) -> ShouldDetach {
        let buffer = args.1.clone();

        let replacement = replacement_of_on_bytes(&args);

        let inserted_text = replacement.inserted_text();

        let extra_replacement = if buffer.has_uneditable_eol()
            && !inserted_text.ends_with('\n')
        {
            let was_empty = buffer.num_bytes() == inserted_text.len() + 1;
            let is_empty = buffer.is_empty();

            if was_empty && !is_empty {
                // If the buffer goes from empty to not empty, the trailing EOL
                // "activates".
                Some(Replacement::insertion(inserted_text.len(), "\n"))
            } else if !was_empty && is_empty {
                // Viceversa, if the buffer goes from not empty to empty, the
                // trailing EOL "deactivates" (but only if the edit didn't
                // already delete it).
                let old_end_col = args.7;
                let did_already_delete_eol = old_end_col == 0;
                (!did_already_delete_eol).then(|| Replacement::deletion(0..1))
            } else {
                None
            }
        } else {
            None
        };

        let buffer_id = BufferId::from(buffer.clone());

        let edited_by = nvim.with_mut(|nvim| {
            nvim.events
                .on_buffer_edited
                .get(&buffer_id)
                .map(|callbacks| callbacks.register_output().agent_id.take())
                .unwrap_or(AgentId::UNKNOWN)
        });

        let edit = Edit {
            made_by: edited_by,
            replacements: iter::once(replacement)
                .chain(extra_replacement)
                .collect(),
        };

        Self::on_edits(nvim, buffer.into(), iter::once(&edit))
    }

    fn on_edits<'a>(
        nvim: &mut impl AccessMut<Neovim>,
        buffer_id: BufferId,
        edits: impl IntoIterator<Item = &'a Edit> + Clone,
    ) -> ShouldDetach {
        nvim.with_mut(|nvim| {
            let Some(mut buffer) = nvim.buffer(buffer_id) else {
                panic!(
                    "callback triggered for an invalid buffer{}",
                    api::Buffer::from(buffer_id)
                        .get_name()
                        .map(|name| format!(": {name}"))
                        .unwrap_or_default()
                );
            };

            let Some(callbacks) = buffer
                .nvim
                .events
                .on_buffer_edited
                .get(&buffer_id)
                .map(|cbs| cbs.cloned())
            else {
                return true;
            };

            for callback in callbacks {
                for edit in edits.clone() {
                    callback((buffer.reborrow(), edit));
                }
            }

            false
        })
    }

    /// Fixes the arguments given to
    /// [`on_bytes`](api::opts::BufAttachOptsBuilder::on_bytes).
    fn fix_on_bytes_args(
        mut args: api::opts::OnBytesArgs,
    ) -> api::opts::OnBytesArgs {
        let (
            _bytes,
            buffer,
            _changedtick,
            start_row,
            start_col,
            _start_offset,
            _old_end_row,
            _old_end_col,
            _old_end_len,
            new_end_row,
            new_end_col,
            new_end_len,
        ) = &mut args;

        // Workaround for edge cases like `dd` in a buffer with a single line
        // or `dG` from the first line in a buffer, where the entire contents
        // of the buffer are deleted but the end position given to `on_bytes`
        // is out of bounds.
        //
        // See https://github.com/neovim/neovim/issues/35557 for an example.
        if (*start_row, *start_col) == (0, 0)
            && (*new_end_row, *new_end_col) == (1, 0)
            && buffer.is_empty()
        {
            *new_end_row = 0;
            *new_end_col = 0;
            *new_end_len = 0;
        }

        args
    }
}

/// The output of the [`BufferEdited::register`] method.
#[derive(Debug, Clone)]
pub(crate) struct BufferEditedRegisterOutput {
    autocmd_ids: [AutocmdId; 2],
    queued_edits: Shared<SmallVec<[Edit; 2]>>,
    agent_id: Rc<Cell<AgentId>>,
}

impl BufferEditedRegisterOutput {
    pub(crate) fn enqueue(&self, edit: Edit) {
        self.queued_edits.with_mut(|vec| vec.push(edit));
    }

    pub(crate) fn set_agent_id(&self, agent_id: AgentId) {
        self.agent_id.set(agent_id);
    }

    pub(crate) fn trigger(&self) {
        let opts = api::opts::ExecAutocmdsOpts::builder()
            .modeline(false)
            .patterns(TRIGGER_AUTOCMD_PATTERN)
            .build();

        api::exec_autocmds(["User"], &opts).expect("couldn't exec autocmd");
    }
}

impl Event for BufferEdited {
    type Args<'a> = (NeovimBuffer<'a>, &'a Edit);
    type Container<'ev> = &'ev mut NoHashMap<BufferId, Callbacks<Self>>;
    type RegisterOutput = BufferEditedRegisterOutput;

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        &mut events.on_buffer_edited
    }

    #[inline]
    fn key(&self) -> BufferId {
        self.0
    }

    #[inline]
    fn kind(&self) -> EventKind {
        EventKind::BufferEdited(*self)
    }

    #[allow(clippy::too_many_lines)]
    #[inline]
    fn register(
        &self,
        events: &Events,
        mut nvim: impl AccessMut<Neovim> + Clone + 'static,
    ) -> Self::RegisterOutput {
        let buffer_id = self.0;
        let queued_edits = Shared::<SmallVec<_>>::default();

        let on_bytes = {
            let mut nvim = nvim.clone();
            let queued_edits = queued_edits.clone();
            move |args: api::opts::OnBytesArgs| {
                let queued_edits = queued_edits.take();
                if !queued_edits.is_empty() {
                    Self::on_edits(&mut nvim, buffer_id, &queued_edits)
                } else {
                    Self::on_bytes(&mut nvim, Self::fix_on_bytes_args(args))
                }
            }
        }
        .catch_unwind()
        .map(|maybe_detach| maybe_detach.unwrap_or(true))
        .into_function();

        api::Buffer::from(buffer_id)
            .attach(
                false,
                &api::opts::BufAttachOpts::builder()
                    .on_bytes(on_bytes)
                    .build(),
            )
            .expect("couldn't attach to buffer");

        let on_fixeol_changed = {
            let mut nvim = nvim.clone();
            move |buffer: api::Buffer, old_value, new_value| {
                debug_assert!(BufferId::from(buffer.clone()) == buffer_id);

                let num_bytes = buffer.num_bytes();

                // Eol-settings don't apply on empty buffers.
                if num_bytes == 0 {
                    return false;
                }

                let replacement = match (old_value, new_value) {
                    // The trailing newline was deleted.
                    (true, false) => {
                        Replacement::deletion(num_bytes..num_bytes + 1)
                    },
                    (false, true) => {
                        Replacement::insertion(num_bytes - 1, "\n")
                    },
                    // The old value is the same as the new one.
                    _ => return false,
                };

                let edit = Edit {
                    made_by: AgentId::UNKNOWN,
                    replacements: smallvec_inline![replacement],
                };

                Self::on_edits(&mut nvim, buffer_id, iter::once(&edit))
            }
        };

        let option_set_autocmd_id = UneditableEndOfLine::on_set(
            events.augroup_id,
            buffer_id,
            on_fixeol_changed,
        );

        #[allow(clippy::redundant_closure)]
        let on_manual_trigger = {
            let queued_edits = queued_edits.clone();
            move |_: api::types::AutocmdCallbackArgs| {
                // Don't call the function if the autocmd is triggered for a
                // different buffer.
                if api::Buffer::from(buffer_id) != api::Buffer::current() {
                    return false;
                }

                let queued = queued_edits.take();

                if !queued.is_empty() {
                    Self::on_edits(&mut nvim, buffer_id, &queued)
                } else {
                    false
                }
            }
        }
        .catch_unwind()
        .map(|maybe_detach| maybe_detach.unwrap_or(true))
        .into_function();

        let user_autocmd_id = api::create_autocmd(
            ["User"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .patterns([TRIGGER_AUTOCMD_PATTERN])
                .callback(on_manual_trigger)
                .build(),
        )
        .expect("couldn't create User autocmd");

        BufferEditedRegisterOutput {
            autocmd_ids: [option_set_autocmd_id, user_autocmd_id],
            queued_edits,
            agent_id: Rc::new(Cell::new(AgentId::UNKNOWN)),
        }
    }

    #[inline]
    fn unregister(output: Self::RegisterOutput) {
        for autocmd_id in output.autocmd_ids {
            let _ = api::del_autocmd(autocmd_id);
        }
    }
}

/// Converts the arguments given to the
/// [`on_bytes`](api::opts::BufAttachOptsBuilder::on_bytes) callback into
/// the corresponding [`Replacement`].
#[inline]
fn replacement_of_on_bytes(args: &api::opts::OnBytesArgs) -> Replacement {
    let &(
        ref _bytes,
        ref buffer,
        _changedtick,
        start_row,
        start_col,
        start_offset,
        _old_end_row,
        _old_end_col,
        old_end_len,
        new_end_row,
        new_end_col,
        new_end_len,
    ) = args;

    let deletion_start = start_offset;

    let deletion_end = start_offset + old_end_len;

    // Fast path for pure deletions.
    if new_end_len == 0 {
        return Replacement::deletion(deletion_start..deletion_end);
    }

    let insertion_start =
        Point { newline_offset: start_row, byte_offset: start_col };

    let insertion_end = Point {
        newline_offset: start_row + new_end_row,
        byte_offset: start_col * (new_end_row == 0) as usize + new_end_col,
    };

    Replacement::new(
        deletion_start..deletion_end,
        &*buffer.get_text_in_point_range(insertion_start..insertion_end),
    )
}
