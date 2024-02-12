use alloc::rc::Rc;

/// TODO: docs
#[derive(Default)]
pub struct Engine {
    notify: Option<Notify>,
}

impl Engine {
    /// TODO: docs
    #[inline(always)]
    pub fn get<'get, T>(&mut self, get: &'get Get<T>) -> &'get T {
        // TODO: track the view that's on the stack, and add `out` to its
        // dependencies.
        //
        get.get_inner()
    }

    /// TODO: docs
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    #[inline(always)]
    pub fn set<T>(&mut self, set: &mut Set<T>, new_value: T) {
        set.set_inner(new_value, self);

        if let Some(notify) = &mut self.notify {
            // SAFETY: we have an exclusive reference to `self`.
            unsafe { notify.notify_unchecked() };
        }
    }

    /// TODO: docs
    #[inline(always)]
    pub fn set_notify<N>(&mut self, notify: N)
    where
        N: FnMut() + 'static,
    {
        self.notify = Some(Notify::new(notify));
    }

    /// TODO: docs
    #[inline(always)]
    pub fn var<T>(&mut self, var: T) -> (Get<T>, Set<T>) {
        let inner = Rc::new(var);
        let get = Get::new(Rc::clone(&inner));
        let set = Set::new(inner);
        (get, set)
    }
}

/// TODO: docs
pub struct Get<T> {
    inner: Rc<T>,
}

impl<T> Get<T> {
    #[inline(always)]
    fn new(inner: Rc<T>) -> Self {
        Self { inner }
    }

    #[inline(always)]
    pub fn get<'this>(&'this self, engine: &mut Engine) -> &'this T {
        engine.get(self)
    }

    #[inline(always)]
    fn get_inner(&self) -> &T {
        &self.inner
    }
}

/// TODO: docs
pub struct Set<T> {
    inner: Rc<T>,
}

impl<T> Set<T> {
    #[inline(always)]
    unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        &mut *(Rc::as_ptr(&self.inner) as *mut _)
    }

    #[inline(always)]
    fn new(inner: Rc<T>) -> Self {
        Self { inner }
    }

    #[inline(always)]
    pub fn set(&mut self, new_value: T, engine: &mut Engine) {
        engine.set(self, new_value)
    }

    #[inline(always)]
    fn set_inner(&mut self, new_value: T, _engine: &mut Engine) {
        // SAFETY: we have an exclusive reference to the `Engine`.
        let inner = unsafe { self.get_mut_unchecked() };
        *inner = new_value;
    }
}

#[derive(Clone)]
struct Notify {
    callback: Rc<dyn FnMut() + 'static>,
}

impl Notify {
    #[inline(always)]
    unsafe fn get_mut_unchecked(&mut self) -> &mut (dyn FnMut() + 'static) {
        &mut *(Rc::as_ptr(&self.callback) as *mut _)
    }

    #[inline(always)]
    fn new<Cb>(callback: Cb) -> Self
    where
        Cb: FnMut() + 'static,
    {
        Self { callback: Rc::new(callback) }
    }

    #[inline(always)]
    unsafe fn notify_unchecked(&mut self) {
        (self.get_mut_unchecked())()
    }
}
