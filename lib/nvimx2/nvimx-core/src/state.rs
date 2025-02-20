use core::any::{self, Any};
use core::cell::Cell;
use core::ops::{Deref, DerefMut};
use std::backtrace::Backtrace;
use std::collections::hash_map::Entry;
use std::panic;

use fxhash::FxHashMap;

use crate::backend::Backend;
use crate::module::{Module, ModuleId};
use crate::notify::Namespace;
use crate::plugin::{PanicInfo, PanicLocation, Plugin, PluginId};
use crate::{NeovimCtx, Shared};

/// TODO: docs.
pub(crate) struct State<B: Backend> {
    backend: B,
    modules: FxHashMap<ModuleId, ModuleState<B>>,
    panic_hook: PanicHook,
}

/// TODO: docs.
pub(crate) struct StateHandle<B: Backend> {
    inner: Shared<State<B>>,
}

/// TODO: docs.
pub(crate) struct StateMut<'a, B: Backend> {
    state: &'a mut State<B>,
    handle: &'a StateHandle<B>,
}

/// A trait implemented by types that can be passed to
/// [`StateMut::with_ctx()`].
pub(crate) trait StateMutWithCtxArgs<B: Backend>: Sized {
    type Output;

    fn call(self, ctx: &mut StateMut<'_, B>) -> Self::Output;
}

struct ModuleState<B: Backend> {
    module: &'static dyn Any,
    panic_handler: Option<&'static dyn PanicHandler<B>>,
}

struct PanicHook {}

trait PanicHandler<B: Backend> {
    fn handle_panic(&self, info: PanicInfo, ctx: &mut NeovimCtx<B>);
}

impl<B: Backend> State<B> {
    #[track_caller]
    #[inline]
    pub(crate) fn add_module<M>(&mut self, module: M) -> &'static M
    where
        M: Module<B>,
    {
        match self.modules.entry(M::id()) {
            Entry::Vacant(entry) => {
                let module = Box::leak(Box::new(module));
                entry.insert(ModuleState { module, panic_handler: None });
                module
            },
            Entry::Occupied(_) => unreachable!(
                "a module of type {:?} has already been added",
                any::type_name::<M>()
            ),
        }
    }

    #[track_caller]
    #[inline]
    pub(crate) fn add_plugin<P>(&mut self, plugin: P) -> &'static P
    where
        P: Plugin<B>,
    {
        match self.modules.entry(<P as Plugin<_>>::id().into()) {
            Entry::Vacant(entry) => {
                let plugin = Box::leak(Box::new(plugin));
                entry.insert(ModuleState {
                    module: plugin,
                    panic_handler: Some(plugin),
                });
                plugin
            },
            Entry::Occupied(_) => unreachable!(
                "a plugin of type {:?} has already been added",
                any::type_name::<P>()
            ),
        }
    }

    #[inline]
    pub(crate) fn get_module<M>(&self) -> Option<&'static M>
    where
        M: Module<B>,
    {
        self.modules.get(&M::id()).map(|module_state| {
            // SAFETY: the ModuleId matched.
            unsafe { downcast_ref_unchecked(module_state.module) }
        })
    }

    #[inline]
    pub(crate) fn new(backend: B) -> Self {
        Self {
            backend,
            modules: FxHashMap::default(),
            panic_hook: PanicHook::set(),
        }
    }
}

impl<B: Backend> StateHandle<B> {
    #[inline]
    pub(crate) fn new(backend: B) -> Self {
        Self { inner: Shared::new(State::new(backend)) }
    }

    #[track_caller]
    #[inline]
    pub(crate) fn with_mut<R>(
        &self,
        f: impl FnOnce(StateMut<'_, B>) -> R,
    ) -> R {
        self.inner.with_mut(|state| f(StateMut { state, handle: self }))
    }
}

impl<B: Backend> StateMut<'_, B> {
    #[inline]
    pub(crate) fn as_mut(&mut self) -> StateMut<'_, B> {
        StateMut { state: self.state, handle: self.handle }
    }

    #[inline]
    pub(crate) fn handle(&self) -> StateHandle<B> {
        self.handle.clone()
    }

    #[inline]
    pub(crate) fn handle_panic(
        &mut self,
        namespace: &Namespace,
        plugin_id: PluginId,
        payload: Box<dyn Any + Send>,
    ) {
        let handler = self
            .modules
            .get(&plugin_id.into())
            .expect("no plugin matching the given PluginId")
            .panic_handler
            .expect("all plugins have panic handlers");
        let info = self.panic_hook.to_info(payload);
        todo!();
        // #[allow(deprecated)]
        // let mut ctx = NeovimCtx::new(namespace, plugin_id, self.as_mut());
        // handler.handle_panic(info, &mut ctx);
    }

    #[track_caller]
    #[inline]
    pub(crate) fn with_ctx<A: StateMutWithCtxArgs<B>>(
        &mut self,
        args: A,
    ) -> A::Output {
        args.call(self)
    }
}

impl PanicHook {
    thread_local! {
        static BACKTRACE: Cell<Option<Backtrace>> = const { Cell::new(None) };
        static LOCATION: Cell<Option<PanicLocation>> = const { Cell::new(None) };
    }

    #[inline]
    fn to_info(&self, payload: Box<dyn Any + Send + 'static>) -> PanicInfo {
        let backtrace = Self::BACKTRACE.with(|b| b.take());
        let location = Self::LOCATION.with(|l| l.take());
        PanicInfo { backtrace, location, payload }
    }

    #[inline]
    fn set() -> Self {
        panic::set_hook({
            Box::new(move |info| {
                let trace = Backtrace::capture();
                let location = info.location().map(Into::into);
                Self::BACKTRACE.with(move |b| b.set(Some(trace)));
                Self::LOCATION.with(move |l| l.set(location));
            })
        });
        Self {}
    }
}

// FIXME: remove once upstream is stabilized.
#[inline]
unsafe fn downcast_ref_unchecked<T: Any>(value: &dyn Any) -> &T {
    debug_assert!(value.is::<T>());
    // SAFETY: caller guarantees that T is the correct type.
    unsafe { &*(value as *const dyn Any as *const T) }
}

impl<B: Backend> Clone for StateHandle<B> {
    #[inline]
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<B: Backend> Deref for State<B> {
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}

impl<B: Backend> DerefMut for State<B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}

impl<B: Backend> Deref for StateMut<'_, B> {
    type Target = State<B>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.state
    }
}

impl<B: Backend> DerefMut for StateMut<'_, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.state
    }
}

impl<F, R, B> StateMutWithCtxArgs<B> for (PluginId, &Namespace, F)
where
    F: FnOnce(&mut NeovimCtx<B>) -> R,
    B: Backend,
{
    type Output = Option<R>;

    #[inline]
    fn call(self, state: &mut StateMut<'_, B>) -> Self::Output {
        let (plugin_id, namespace, callback) = self;
        #[allow(deprecated)]
        let mut ctx = NeovimCtx::new(namespace, state.as_mut());
        match panic::catch_unwind(panic::AssertUnwindSafe(|| {
            callback(&mut ctx)
        })) {
            Ok(ret) => Some(ret),
            Err(payload) => {
                state.handle_panic(namespace, plugin_id, payload);
                None
            },
        }
    }
}

impl<F, R, B> StateMutWithCtxArgs<B> for (&Namespace, F)
where
    F: FnOnce(&mut NeovimCtx<B>) -> R,
    B: Backend,
{
    type Output = R;

    #[inline]
    fn call(self, state: &mut StateMut<'_, B>) -> Self::Output {
        let (namespace, callback) = self;
        #[allow(deprecated)]
        callback(&mut NeovimCtx::new(namespace, state.as_mut()))
    }
}

impl<F, R, B> StateMutWithCtxArgs<B> for F
where
    F: FnOnce(&mut NeovimCtx<B>) -> R,
    B: Backend,
{
    type Output = R;

    #[inline]
    fn call(self, state: &mut StateMut<'_, B>) -> Self::Output {
        #[allow(deprecated)]
        self(&mut NeovimCtx::new(&Namespace::default(), state.as_mut()))
    }
}

impl<P, B> PanicHandler<B> for P
where
    P: Plugin<B>,
    B: Backend,
{
    #[inline]
    fn handle_panic(&self, info: PanicInfo, ctx: &mut NeovimCtx<B>) {
        Plugin::handle_panic(self, info, ctx);
    }
}
