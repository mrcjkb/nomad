use core::fmt::Display;

#[derive(Default)]
pub(crate) struct Warning {}

impl Warning {
    pub(crate) fn msg(self, _msg: WarningMsg) -> Self {
        todo!();
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn print(self) {}
}

#[derive(Default)]
pub(crate) struct WarningMsg {}

impl WarningMsg {
    #[inline]
    pub(crate) fn add<C: Chunk>(&mut self, _chunk: C) -> &mut Self {
        self
    }

    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

/// TODO: docs
pub(crate) trait Chunk: Sized {
    #[inline]
    fn highlight(self) -> Highlight<Self> {
        Highlight::new(self)
    }
}

impl Chunk for &str {}

/// TODO: docs
pub(crate) struct Highlight<C> {
    chunk: C,
}

impl<C> Highlight<C> {
    #[inline]
    pub(crate) fn new(chunk: C) -> Self {
        Self { chunk }
    }
}

impl<C: Chunk> Chunk for Highlight<C> {}
