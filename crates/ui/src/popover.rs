use core::marker::PhantomData;

use crate::{Render, View};

/// TODO: docs
pub struct Popover {
    /// TODO: docs
    anchor: PopoverAnchor,

    /// TODO: docs
    root: Box<dyn Render + 'static>,

    /// TODO: docs
    window: View,
}

impl Popover {
    /// TODO: docs
    #[inline]
    pub fn builder() -> PopoverBuilder<RootRender> {
        PopoverBuilder { popover: Self::uninit(), _state: PhantomData }
    }

    #[inline]
    fn uninit() -> Self {
        Self {
            anchor: PopoverAnchor::Editor,
            root: Box::new(()),
            window: View::new_hidden(),
        }
    }
}

/// TODO: docs
pub enum PopoverAnchor {
    /// TODO: docs
    Cursor,

    /// TODO: docs
    Editor,
}

/// TODO: docs
pub struct PopoverBuilder<State> {
    popover: Popover,
    _state: PhantomData<State>,
}

impl PopoverBuilder<RootRender> {
    /// TODO: docs
    #[inline]
    pub fn render<R>(mut self, root: R) -> PopoverBuilder<Anchor>
    where
        R: Render + 'static,
    {
        self.popover.root = Box::new(root);
        PopoverBuilder { popover: self.popover, _state: PhantomData }
    }
}

impl PopoverBuilder<Anchor> {
    /// TODO: docs
    #[inline]
    pub fn anchor<A>(mut self, anchor: A) -> PopoverBuilder<Done>
    where
        A: Into<PopoverAnchor>,
    {
        self.popover.anchor = anchor.into();
        PopoverBuilder { popover: self.popover, _state: PhantomData }
    }
}

impl PopoverBuilder<Done> {
    /// TODO: docs
    #[inline]
    pub fn open(self) -> Popover {
        self.popover
    }
}

use typestate::*;

mod typestate {
    /// TODO: docs.
    pub struct Anchor;

    /// TODO: docs.
    pub struct RootRender;

    /// TODO: docs.
    pub struct Done;
}
