use core::ops::{Deref, DerefMut};

use crate::incremental;

/// TODO: docs
pub struct Engine {
    inner: incremental::Engine,
}

impl Engine {
    #[inline(always)]
    fn new() -> Self {
        Self { inner: incremental::Engine::new() }
    }

    /// TODO: docs
    #[inline(always)]
    pub fn set_notify<N>(&mut self, notify: N)
    where
        N: FnMut() + 'static,
    {
        self.inner.set_notify(notify);
    }

    /// TODO: docs
    #[inline(always)]
    pub fn var<T>(&mut self, var: T) -> (Get<T>, Set<T>) {
        let (get, set) = self.inner.var(var);
        (Get { inner: get }, Set { inner: set })
    }
}

/// TODO: docs
pub struct Get<T> {
    inner: incremental::Get<T>,
}

impl<T> Get<T> {
    /// TODO: docs
    #[inline(always)]
    pub fn get(&self, ctx: &mut ViewCtx) -> &T {
        self.inner.get(&mut ctx.engine.inner)
    }
}

pub struct Set<T> {
    inner: incremental::Set<T>,
}

impl<T> Set<T> {
    /// TODO: docs
    #[inline(always)]
    pub fn set(&mut self, new_value: T, ctx: &mut SetCtx) {
        self.inner.set(new_value, &mut ctx.engine.inner)
    }
}

pub struct GetCtx {
    engine: Engine,
}

impl GetCtx {
    #[inline(always)]
    pub fn from_ref_mut(engine_ref: &mut Engine) -> &mut Self {
        unsafe { core::mem::transmute(engine_ref) }
    }
}

pub struct SetCtx {
    engine: Engine,
}

impl SetCtx {
    #[inline(always)]
    pub fn from_ref_mut(engine_ref: &mut Engine) -> &mut Self {
        unsafe { core::mem::transmute(engine_ref) }
    }
}

pub struct ViewCtx {
    get_ctx: GetCtx,
}

impl ViewCtx {
    #[inline(always)]
    pub(crate) fn from_ref_mut(engine_ref: &mut Engine) -> &mut Self {
        let get_ctx = GetCtx::from_ref_mut(engine_ref);
        unsafe { core::mem::transmute(get_ctx) }
    }
}

impl Deref for ViewCtx {
    type Target = GetCtx;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.get_ctx
    }
}

impl DerefMut for ViewCtx {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.get_ctx
    }
}

pub trait Platform {
    type Surface: ?Sized;

    #[inline(always)]
    fn engine() -> Engine {
        Engine::new()
    }
}

/// TODO: docs
pub trait View<P: Platform> {
    fn view(&self, ctx: &mut ViewCtx) -> impl Render<P>;
}

/// TODO: docs
pub trait Render<P: Platform>: 'static {
    /// TODO: docs
    fn render(&self, surface: &mut P::Surface);
}
