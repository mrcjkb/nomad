use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Metric;

/// TODO: docs
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cells(u32);

impl From<u32> for Cells {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Cells> for u32 {
    #[inline]
    fn from(cells: Cells) -> Self {
        cells.0
    }
}

impl Add for Cells {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl AddAssign for Cells {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Cells {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl SubAssign for Cells {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Metric for Cells {
    #[inline]
    fn zero() -> Self {
        Self(0)
    }
}
