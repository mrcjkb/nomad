use core::ops::{Bound, Range, RangeBounds};

use nvim::api::{self, opts};

use crate::{ByteOffset, Edit, FromCtx, IntoCtx, Point, Replacement, Shared};

type OnEdit = Box<dyn FnMut(&Replacement<ByteOffset>) + 'static>;

/// A handle to a Neovim buffer.
#[cfg_attr(not(feature = "tests"), doc(hidden))]
#[derive(Clone)]
pub struct NvimBuffer {
    /// The buffer handle.
    inner: api::Buffer,

    /// The list of callbacks to be called every time the buffer is edited.
    on_edit_callbacks: Shared<Vec<OnEdit>>,
}

impl core::fmt::Debug for NvimBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NvimBuffer").field(&self.inner).finish()
    }
}

impl NvimBuffer {
    /// Creates a new buffer.
    #[inline]
    pub fn create() -> Self {
        let Ok(buf) = api::create_buf(true, false) else { unreachable!() };
        let Ok(buf) = Self::new(buf) else { unreachable!() };
        buf
    }

    /// Edits the buffer.
    #[inline]
    pub fn edit<E>(&mut self, edit: E) -> E::Diff
    where
        E: Edit<Self>,
    {
        edit.apply(self)
    }

    /// TODO: docs.
    #[inline]
    fn end_point(&self) -> Point<ByteOffset> {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn get<R, O>(&self, range: R) -> Result<String, api::Error>
    where
        R: RangeBounds<O>,
        O: IntoCtx<Point<ByteOffset>, Self> + Copy,
    {
        let start = match range.start_bound() {
            Bound::Included(&start) => start.into_ctx(self),
            Bound::Excluded(&start) => start.into_ctx(self),
            Bound::Unbounded => Point::default(),
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => end.into_ctx(self),
            Bound::Excluded(&end) => end.into_ctx(self),
            Bound::Unbounded => self.end_point(),
        };

        self.get_point_range(start..end)
    }

    /// TODO: docs
    #[inline]
    fn get_point_range(
        &self,
        range: Range<Point<ByteOffset>>,
    ) -> Result<String, api::Error> {
        let mut lines = self.inner.get_text(
            range.start.line()..range.end.line(),
            range.start.offset().into(),
            range.end.offset().into(),
            &Default::default(),
        )?;

        let mut text = String::new();

        let Some(first_line) = lines.next() else {
            return Ok(text);
        };

        text.push_str(&first_line.to_string_lossy());

        for line in lines {
            text.push('\n');
            text.push_str(&line.to_string_lossy());
        }

        Ok(text)
    }

    /// Registers a callback to be called every time the buffer is edited.
    #[inline]
    pub fn on_edit<F: FnMut(&Replacement<ByteOffset>) + 'static>(
        &self,
        callback: F,
    ) {
        self.on_edit_callbacks
            .with_mut(|callbacks| callbacks.push(Box::new(callback)));
    }

    #[inline]
    fn new(buffer: api::Buffer) -> Result<Self, NvimBufferDoesntExistError> {
        let on_edit_callbacks = Shared::<Vec<OnEdit>>::default();

        let cbs = on_edit_callbacks.clone();

        let opts = opts::BufAttachOpts::builder()
            .on_bytes(move |args| {
                let edit = Replacement::from(args);
                cbs.with_mut(|cbs| cbs.iter_mut().for_each(|cb| cb(&edit)));
                Ok(false)
            })
            .build();

        buffer
            .attach(false, &opts)
            // All the arguments passed to `attach()` are valid, so if it fails
            // it must be because the buffer doesn't exist.
            .map_err(|_| NvimBufferDoesntExistError)?;

        Ok(Self { inner: buffer, on_edit_callbacks })
    }

    #[inline]
    fn point_of_offset(
        &self,
        offset: ByteOffset,
    ) -> Result<Point<ByteOffset>, api::Error> {
        todo!();
    }

    /// TODO: docs
    #[inline]
    fn replace_point_range(
        &mut self,
        range: Range<Point<ByteOffset>>,
        replacement: impl Iterator<Item = nvim::String>,
    ) -> Result<(), api::Error> {
        self.inner.set_text(
            range.start.line()..range.end.line(),
            range.start.offset().into(),
            range.end.offset().into(),
            replacement,
        )
    }
}

impl<Offset> Edit<NvimBuffer> for &Replacement<Offset>
where
    Offset: IntoCtx<Point<ByteOffset>, NvimBuffer> + Copy,
{
    type Diff = ();

    #[inline]
    fn apply(self, buf: &mut NvimBuffer) -> Self::Diff {
        let start = self.start().into_ctx(buf);
        let end = self.end().into_ctx(buf);
        let replacement = core::iter::once(self.replacement().into());
        let _ = buf.replace_point_range(start..end, replacement);
    }
}

impl<Offset> Edit<NvimBuffer> for Replacement<Offset>
where
    Offset: IntoCtx<Point<ByteOffset>, NvimBuffer> + Copy,
{
    type Diff = ();

    #[inline]
    fn apply(self, buf: &mut NvimBuffer) -> Self::Diff {
        (&self).apply(buf)
    }
}

impl FromCtx<ByteOffset, NvimBuffer> for Point<ByteOffset> {
    #[inline]
    fn from_ctx(offset: ByteOffset, buf: &NvimBuffer) -> Self {
        match buf.point_of_offset(offset) {
            Ok(point) => point,
            Err(err) => panic!("couldn't convert offset to point: {err}"),
        }
    }
}

impl From<opts::OnBytesArgs> for Replacement<ByteOffset> {
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
        ): opts::OnBytesArgs,
    ) -> Self {
        todo!();
        // let replacement_start = Point { row: start_row, col: start_col };
        //
        // let replacement_end = Point {
        //     row: start_row + new_end_row,
        //     col: start_col * (new_end_row == 0) as usize + new_end_col,
        // };
        //
        // let replacement = if replacement_start == replacement_end {
        //     String::new()
        // } else {
        //     nvim_buf_get_text(&buf, replacement_start..replacement_end)
        //         .expect("buffer must exist")
        // };
        //
        // Self {
        //     start: start_offset,
        //     end: start_offset + old_end_len,
        //     replacement,
        // }
    }
}

/// An error returned whenever a..
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NvimBufferDoesntExistError;
