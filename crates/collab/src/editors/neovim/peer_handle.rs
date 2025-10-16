use core::cell::{Cell, LazyCell};
use core::iter;

use collab_types::{Peer, PeerHandle};
use editor::ByteOffset;
use neovim::buffer::BufferExt;
use neovim::oxi::api;

use crate::editors::neovim::PeerHighlightGroup;

thread_local! {
    /// The highlight group ID of the `Normal` highlight group.
    static NORMAL_HL_GROUP_ID: LazyCell<u32> = const { LazyCell::new(|| {
        api::get_hl_id_by_name("Normal")
            .expect("couldn't get highlight group ID for 'Normal'")
    }) };
}

/// A remote peer's handle in a buffer, displayed either directly above or
/// below their [`cursor`](crate::editors::neovim::PeerCursor).
pub struct NeovimPeerHandle {
    /// The buffer the cursor is in.
    buffer: api::Buffer,

    /// The ID of the extmark used to display the handle.
    extmark_id: u32,

    /// The ID of the highlight group used to highlight the handle.
    hl_group_id: u32,

    /// The ID of the namespace the [`extmark_id`](Self::extmark_id) belongs
    /// to.
    namespace_id: u32,

    /// The remote peer's handle.
    peer_handle: PeerHandle,
}

/// The highlight group used to highlight a remote peer's handle.
pub(super) struct PeerHandleHighlightGroup;

impl PeerHandleHighlightGroup {
    thread_local! {
        static GROUP_IDS: Cell<[u32; 16]> = const { Cell::new([0; _]) };
    }
}

impl NeovimPeerHandle {
    /// Creates a new handle for the given remote peer to be displayed above or
    /// below the cursor at the given offset in the given buffer.
    pub(super) fn create(
        peer: Peer,
        mut buffer: api::Buffer,
        cursor_offset: ByteOffset,
        namespace_id: u32,
    ) -> Self {
        let hl_group_id = PeerHandleHighlightGroup::group_id(peer.id);

        let (line, col, mut opts_builder) = Self::extmark_params(
            buffer.clone(),
            cursor_offset,
            &peer.handle,
            hl_group_id,
        );

        let extmark_id = buffer
            .set_extmark(namespace_id, line, col, &opts_builder.build())
            .expect("couldn't create extmark");

        Self {
            buffer,
            extmark_id,
            hl_group_id,
            namespace_id,
            peer_handle: peer.handle,
        }
    }

    /// Moves the handle to keep it in sync with the new cursor offset.
    pub(super) fn r#move(&mut self, new_cursor_offset: ByteOffset) {
        let (line, col, mut opts_builder) = Self::extmark_params(
            self.buffer.clone(),
            new_cursor_offset,
            &self.peer_handle,
            self.hl_group_id,
        );

        let opts = opts_builder.id(self.extmark_id).build();

        let new_extmark_id = self
            .buffer
            .set_extmark(self.namespace_id, line, col, &opts)
            .expect("couldn't move extmark");

        debug_assert_eq!(new_extmark_id, self.extmark_id);
    }

    /// Removes the handle from the buffer.
    pub(super) fn remove(mut self) {
        self.buffer
            .del_extmark(self.namespace_id, self.extmark_id)
            .expect("couldn't delete extmark");
    }

    /// Returns the line, column, and options to give to
    /// [`api::Buffer::set_extmark`] to position the peer handle above or below
    /// the cursor at the given byte offset.
    fn extmark_params(
        buffer: api::Buffer,
        cursor_offset: ByteOffset,
        peer_handle: &PeerHandle,
        hl_group_id: u32,
    ) -> (usize, usize, api::opts::SetExtmarkOptsBuilder) {
        let cursor_point = buffer.point_of_byte(cursor_offset);

        let num_rows = buffer.num_rows();

        let line_idx =
            // If the cursor is not on the first line, place the handle on the
            // previous line so that it appears above the cursor.
            if cursor_point.newline_offset > 0 {
                cursor_point.newline_offset - 1
            }
            // Otherwise, try to place it on the next line so that it appears
            // below the cursor.
            else if num_rows > 1 {
                cursor_point.newline_offset + 1
            }
            // If the buffer has a single line, we'll use the virt_lines
            // approach to display the handle on a virtual line below the
            // cursor.
            else {
                0
            };

        let use_virt_lines = num_rows == 1;

        let line_len = buffer.num_bytes_in_line_after(line_idx);

        // FIXME: using the cursor's offset as the target column for the handle
        // could result in the handle being vertically misaligned with the
        // cursor if either line contains multi-byte characters.
        //
        // We could use `nvim_strwidth` to go from byte offset -> visual
        // column, but I don't think Neovim exposes a function which does
        // the opposite (visual column -> byte offset).
        //
        // FIXME: this also doesn't handle tabs correctly. For those, we'd have
        // to count the number of preceding tabs in the cursor and target
        // lines, also taking into account the 'tabstop' option.
        let target_col = cursor_point.byte_offset;

        // Clamp the column to the length of the line.
        let col = target_col.min(line_len);

        let num_padding_spaces = if use_virt_lines {
            // When setting virt_lines, the padding always has to match the
            // target column.
            target_col
        } else {
            // If the previous/next line is shorter than target column, we'll
            // compensate by adding some padding spaces to the virt_text.
            //
            // For example, if the buffer is:
            //
            // ```
            // foo
            // Hello |World!
            // ```
            //
            // Where the '|' represents the remote peer's cursor, then the
            // extmark will be placed at the end of the first line (line=0,
            // col=3).
            //
            // But the cursor is at column 6 on the second line, so we need to
            // add 3 spaces of padding to the extmark's virt_text to make the
            // handle appear to be vertically aligned with the cursor.
            target_col - col
        };

        let padding_chunk = if num_padding_spaces > 0 {
            let spaces = " ".repeat(num_padding_spaces);
            Some((spaces, NORMAL_HL_GROUP_ID.with(|id| **id)))
        } else {
            None
        };

        let chunks = padding_chunk
            .into_iter()
            .chain(iter::once((format!(" {peer_handle} "), hl_group_id)));

        let mut opts_builder = api::opts::SetExtmarkOpts::builder();

        if use_virt_lines {
            opts_builder.virt_lines(iter::once(chunks));
        } else {
            opts_builder
                .virt_text(chunks)
                .virt_text_pos(api::types::ExtmarkVirtTextPosition::Overlay);
        }

        (line_idx, col, opts_builder)
    }
}

impl PeerHighlightGroup for PeerHandleHighlightGroup {
    const NAME_PREFIX: &str = "NomadCollabPeerHandle";

    fn set_hl_opts() -> api::opts::SetHighlightOpts {
        api::opts::SetHighlightOpts::builder().link("PmenuSel").build()
    }

    fn with_group_ids<R>(fun: impl FnOnce(&[Cell<u32>]) -> R) -> R {
        Self::GROUP_IDS.with(|ids| fun(ids.as_array_of_cells().as_slice()))
    }
}
