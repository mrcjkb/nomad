use core::fmt::Debug;

/// TODO: docs
pub trait Metric: Debug + Copy + Eq + Ord {
    /// TODO: docs
    fn zero() -> Self;

    /// TODO: docs
    #[inline]
    fn is_zero(self) -> bool {
        self == Self::zero()
    }
}

impl Metric for usize {
    #[inline]
    fn zero() -> Self {
        0
    }
}
