use common::Sender;

use crate::*;

type OnExit = Box<dyn FnOnce(Option<FuzzyItem>) + 'static>;
type OnSelect = Box<dyn FnMut(&FuzzyItem) + 'static>;
type OnConfirm = Box<dyn FnOnce(FuzzyItem) + 'static>;

pub struct FuzzyBuilder {
    config: FuzzyConfig,
    sender: Sender<Message>,
}

/// TODO: docs
#[derive(Default)]
pub struct FuzzyConfig {
    pub(crate) items: Vec<FuzzyItem>,
    pub(crate) on_confirm: Option<OnConfirm>,
    pub(crate) on_cancel: Option<OnExit>,
    pub(crate) on_select: Option<OnSelect>,
    pub(crate) starting_text: Option<String>,
    pub(crate) starting_selected: Option<usize>,
}

impl FuzzyBuilder {
    /// The function that's called when the user confirms an item.
    ///
    /// The argument of the function is the item that was confirmed.
    pub fn on_confirm<F>(mut self, fun: F) -> Self
    where
        F: FnOnce(FuzzyItem) + 'static,
    {
        self.config.on_confirm = Some(Box::new(fun));
        self
    }

    /// The function that's called when the user exits the modal without
    /// confirming an item.
    ///
    /// The argument of the function is the item that was selected when the
    /// modal was exited (if there was one).
    pub fn on_cancel<F>(mut self, fun: F) -> Self
    where
        F: FnOnce(Option<FuzzyItem>) + 'static,
    {
        self.config.on_cancel = Some(Box::new(fun));
        self
    }

    /// The function that's called when the user selects an item.
    ///
    /// The argument of the function is the item that was selected.
    ///
    /// Note that selecting an item is different from confirming an item.
    /// Selecting simply means that the user has scrolled to an item and is
    /// currently hovering over it.
    pub fn on_select<F>(mut self, fun: F) -> Self
    where
        F: FnMut(&FuzzyItem) + 'static,
    {
        self.config.on_select = Some(Box::new(fun));
        self
    }

    /// TODO: docs
    pub fn open(self) -> FuzzyHandle {
        let Self { sender, config } = self;
        sender.send(Message::Open(config));
        FuzzyHandle::new(sender)
    }

    /// TODO: docs
    pub fn open_with_selected(
        mut self,
        selected_item_idx: usize,
    ) -> FuzzyHandle {
        self.config.starting_selected = Some(selected_item_idx);
        self.open()
    }

    pub(crate) fn new(sender: Sender<Message>) -> Self {
        Self { sender, config: FuzzyConfig::default() }
    }

    /// TODO: docs
    pub fn with_items<Item, Items>(mut self, items: Items) -> Self
    where
        Item: Into<FuzzyItem>,
        Items: IntoIterator<Item = Item>,
    {
        self.config.items.extend(items.into_iter().map(Into::into));
        self
    }

    /// Set the default text that's displayed on the query line when there's no
    /// query.
    ///
    /// # Panics
    ///
    /// Panics if the text contains a newline.
    pub fn with_starting_text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        assert!(!text.contains('\n'));
        self.config.starting_text = Some(text);
        self
    }
}
