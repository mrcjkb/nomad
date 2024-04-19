/// A stateful version of the [`From`] trait.
pub trait FromCtx<T, Ctx: ?Sized> {
    /// Turns the value into `Self` using a context.
    fn from_ctx(value: T, ctx: &Ctx) -> Self;
}

impl<T, Ctx> FromCtx<T, Ctx> for T {
    #[inline]
    fn from_ctx(value: T, _: &Ctx) -> Self {
        value
    }
}

/// A stateful version of the [`Into`] trait.
pub trait IntoCtx<T, Ctx: ?Sized> {
    /// Turns `Self` into the value using a context.
    fn into_ctx(self, ctx: &Ctx) -> T;
}

impl<Ctx, T, U> IntoCtx<U, Ctx> for T
where
    U: FromCtx<T, Ctx>,
{
    #[inline]
    fn into_ctx(self, ctx: &Ctx) -> U {
        U::from_ctx(self, ctx)
    }
}
