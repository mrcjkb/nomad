use alloc::borrow::Cow;
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Bound, Range, RangeBounds};

use collab_fs::AbsUtf8Path;
use nvim_oxi::api::{self, Buffer as NvimBuffer};

use super::Neovim;
use crate::{ByteOffset, Text};

/// TODO: docs.
pub struct Buffer {
    id: BufferId,
}

/// TODO: docs.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BufferId {
    inner: NvimBuffer,
}

/// The 2D equivalent of a `ByteOffset`.
struct Point {
    /// The index of the line in the buffer.
    line_idx: usize,

    /// The byte offset in the line.
    byte_offset: ByteOffset,
}

impl Buffer {
    pub(super) fn new(id: BufferId) -> Self {
        Self { id }
    }

    fn as_nvim(&self) -> &NvimBuffer {
        &self.id.inner
    }

    #[track_caller]
    fn byte_offset_of_point(&self, point: Point) -> ByteOffset {
        todo!()
    }

    #[track_caller]
    fn get_text_in_point_range(&self, point_range: Range<Point>) -> Text {
        todo!()
    }

    #[track_caller]
    fn point_of_byte_offset(&self, byte_offset: ByteOffset) -> Point {
        todo!()
    }

    fn point_of_eof(&self) -> Point {
        fn point_of_eof(buffer: &Buffer) -> Result<Point, api::Error> {
            let buf = buffer.as_nvim();

            let num_lines = buf.line_count()?;

            if num_lines == 0 {
                return Ok(Point::zero());
            }

            let last_line_len = buf.get_offset(num_lines)?
            // `get_offset(line_count)` seems to always include the trailing
            // newline, even when `eol` is turned off.
            //
            // TODO: shouldn't we only correct this is `eol` is turned off?
            - 1
            - buf.get_offset(num_lines - 1)?;

            Ok(Point {
                line_idx: num_lines - 1,
                byte_offset: ByteOffset::new(last_line_len),
            })
        }

        match point_of_eof(self) {
            Ok(point) => point,
            Err(_) => panic!("{self:?} has been deleted"),
        }
    }

    #[track_caller]
    fn point_range_of_byte_range<R>(&self, byte_range: R) -> Range<Point>
    where
        R: RangeBounds<ByteOffset>,
    {
        let start = match byte_range.start_bound() {
            Bound::Excluded(&start) | Bound::Included(&start) => {
                self.point_of_byte_offset(start)
            },
            Bound::Unbounded => Point::zero(),
        };

        let end = match byte_range.end_bound() {
            Bound::Excluded(&end) => self.point_of_byte_offset(end),
            Bound::Included(&end) => self.point_of_byte_offset(end + 1),
            Bound::Unbounded => self.point_of_eof(),
        };

        start..end
    }

    #[track_caller]
    fn replace_text_in_point_range(
        &self,
        point_range: Range<Point>,
        replacement: &str,
    ) {
        todo!()
    }
}

impl crate::Buffer<Neovim> for Buffer {
    type Id = BufferId;

    fn byte_len(&self) -> usize {
        todo!()
    }

    fn get_text<R>(&self, byte_range: R) -> Text
    where
        R: RangeBounds<ByteOffset>,
    {
        let point_range = self.point_range_of_byte_range(byte_range);
        self.get_text_in_point_range(point_range)
    }

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn path(&self) -> Option<Cow<'_, AbsUtf8Path>> {
        todo!()
    }

    fn set_text<R, T>(&mut self, replaced_range: R, new_text: T)
    where
        R: RangeBounds<ByteOffset>,
        T: AsRef<str>,
    {
        let point_range = self.point_range_of_byte_range(replaced_range);
        self.replace_text_in_point_range(point_range, new_text.as_ref());
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Buffer").field(&self.as_nvim().handle()).finish()
    }
}

impl BufferId {
    pub(super) fn is_of_text_buffer(&self) -> bool {
        todo!();
    }

    pub(super) fn new(inner: NvimBuffer) -> Self {
        Self { inner }
    }
}

impl PartialOrd for BufferId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BufferId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.handle().cmp(&other.inner.handle())
    }
}

impl Point {
    fn zero() -> Self {
        Self { line_idx: 0, byte_offset: ByteOffset::new(0) }
    }
}
