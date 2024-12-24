/// TODO: docs.
pub struct ModuleApi<M, B> {
    module: M,
    backend: B,
}

impl<M, B> ModuleApi<M, B> {
    /// TODO: docs.
    #[inline]
    pub fn with_command<C>(self, cmd: C) -> Self
    where
        C: Command<Module = M>,
    {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn with_default_command<C>(self, cmd: C) -> Self
    where
        C: Command<Module = M>,
    {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn with_default_function<F>(self, fun: F) -> Self
    where
        F: Function<Module = M>,
    {
        todo!();
    }

    /// TODO: docs.
    #[inline]
    pub fn with_function<F>(self, fun: F) -> Self
    where
        F: Function<Module = M>,
    {
        todo!();
    }
}
