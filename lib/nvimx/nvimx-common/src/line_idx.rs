use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Metric;

/// A line index a buffer.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineIdx(usize);

impl LineIdx {
    /// Returns the line index as a `usize`.
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Creates a new `LineIdx` with the given index.
    #[inline]
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }
}

impl Add<Self> for LineIdx {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.as_usize() + other.as_usize())
    }
}

impl AddAssign<Self> for LineIdx {
    #[inline]
    fn add_assign(&mut self, idx: Self) {
        self.0 += idx.as_usize();
    }
}

impl Sub<Self> for LineIdx {
    type Output = Self;

    #[inline]
    fn sub(self, idx: Self) -> Self {
        Self(self.as_usize() - idx.as_usize())
    }
}

impl SubAssign<Self> for LineIdx {
    #[inline]
    fn sub_assign(&mut self, idx: Self) {
        self.0 -= idx.as_usize();
    }
}

impl From<usize> for LineIdx {
    #[inline]
    fn from(idx: usize) -> Self {
        Self::new(idx)
    }
}

impl From<LineIdx> for usize {
    #[inline]
    fn from(idx: LineIdx) -> usize {
        idx.as_usize()
    }
}

impl Metric for LineIdx {
    #[inline]
    fn zero() -> Self {
        Self(0)
    }
}
