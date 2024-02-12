use alloc::rc::Rc;
use core::cell::RefCell;
use core::convert::Infallible;
use core::marker::PhantomData;

use nvim_oxi::api::types::AutocmdCallbackArgs;
use nvim_oxi::api::{self, opts};
use nvim_oxi::libuv::AsyncHandle;

use crate::react::{
    Engine,
    Get,
    Platform,
    Render,
    Set,
    SetCtx,
    View,
    ViewCtx,
};

/// TODO: docs
//
// where can we get callbacks or ways to set the code?
//
// autocmds
// commands
// mappings
pub struct Neovim {
    engine: Rc<RefCell<Engine>>,
}

impl Platform for Neovim {
    type Surface = ();
}

impl Neovim {
    #[inline(always)]
    pub fn autocmd_builder(&mut self) -> AutocmdBuilder<'_, WantsEvent> {
        AutocmdBuilder::new(self)
    }

    /// TODO: docs
    #[inline]
    pub fn mount<RootView, MountRoot>(mount_root: MountRoot)
    where
        MountRoot: FnOnce(&mut Self) -> RootView,
        RootView: View<Self> + 'static,
    {
        let mut this = Self::new();

        let root_view = mount_root(&mut this);

        let Self { engine } = this;

        let also_engine = Rc::clone(&engine);

        let async_handle = AsyncHandle::new(move || {
            let engine = &mut *engine.borrow_mut();

            let view_ctx = ViewCtx::from_ref_mut(engine);

            let renderable = root_view.view(view_ctx);

            nvim_oxi::schedule(move |_| {
                renderable.render(&mut ());
                Ok(())
            });

            Ok::<_, Infallible>(())
        })
        .unwrap();

        let engine = &mut *also_engine.borrow_mut();

        engine.set_notify(move || async_handle.send().unwrap());
    }

    /// TODO: docs
    #[inline(always)]
    fn new() -> Self {
        Self { engine: Rc::new(RefCell::new(Self::engine())) }
    }

    /// TODO: docs
    #[inline]
    pub fn var<T>(&mut self, var: T) -> (Get<T>, Set<T>) {
        let engine = &mut *self.engine.borrow_mut();
        engine.var(var)
    }
}

/// TODO: docs
pub enum AutocmdEvent {
    /// TODO: docs
    CursorMoved,

    /// TODO: docs
    CursorMovedI,
}

impl AutocmdEvent {
    fn as_str(self) -> &'static str {
        match self {
            Self::CursorMoved => "CursorMoved",
            Self::CursorMovedI => "CursorMovedI",
        }
    }
}

/// TODO: docs
pub struct AutocmdId(u32);

/// TODO: docs
pub struct AutocmdBuilder<'nvim, State> {
    builder: opts::CreateAutocmdOptsBuilder,
    events: Vec<AutocmdEvent>,
    nvim: &'nvim mut Neovim,
    _state: PhantomData<State>,
}

impl<'nvim> AutocmdBuilder<'nvim, WantsEvent> {
    #[inline(always)]
    pub fn build(self) -> AutocmdId {
        let Self { mut builder, events, .. } = self;
        let events = events.into_iter().map(AutocmdEvent::as_str);
        let id = api::create_autocmd(events, &builder.build()).unwrap();
        AutocmdId(id)
    }

    #[inline(always)]
    pub fn exec<Cb>(mut self, mut callback: Cb) -> Self
    where
        Cb: FnMut(AutocmdCallbackArgs, &mut SetCtx) + 'static,
    {
        let engine = Rc::clone(&self.nvim.engine);

        self.builder.callback(move |args| {
            let engine = &mut *engine.borrow_mut();
            let set_ctx = SetCtx::from_ref_mut(engine);
            callback(args, set_ctx);
            Ok::<_, Infallible>(false)
        });

        self
    }

    #[inline(always)]
    fn new(nvim: &'nvim mut Neovim) -> Self {
        Self {
            nvim,
            events: Vec::new(),
            builder: Default::default(),
            _state: PhantomData,
        }
    }

    #[inline(always)]
    pub fn on_event(mut self, event: AutocmdEvent) -> Self {
        self.events.push(event);
        self
    }
}

/// TODO: docs
pub struct WantsEvent;

/// TODO: docs
pub struct WithEvent;
