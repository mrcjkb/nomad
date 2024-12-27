use core::convert::Infallible;

use super::{Level, Message};

/// TODO: docs.
pub trait Error: 'static {
    /// TODO: docs.
    fn to_severity(&self) -> Option<Level>;

    /// TODO: docs.
    fn to_message(&self) -> Message;
}

impl Error for Infallible {
    fn to_severity(&self) -> Option<Level> {
        unreachable!()
    }

    fn to_message(&self) -> Message {
        unreachable!()
    }
}

impl Error for Box<dyn Error> {
    fn to_severity(&self) -> Option<Level> {
        (**self).to_severity()
    }

    fn to_message(&self) -> Message {
        (**self).to_message()
    }
}
