use common::Sender;

use crate::*;

pub struct FuzzyHandle {
    sender: Sender<Message>,
}

impl FuzzyHandle {
    /// TODO: docs
    pub fn add_results(&self, results: Vec<FuzzyItem>) {
        self.send(Message::AddResults(results))
    }

    /// TODO: docs
    pub fn close(self) {
        self.send(Message::Close)
    }

    pub(crate) fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    fn send(&self, msg: Message) {
        self.sender.send(msg)
    }
}
