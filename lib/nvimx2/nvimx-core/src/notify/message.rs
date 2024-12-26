use core::fmt;

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {}

impl fmt::Display for Message {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: Message")
    }
}
