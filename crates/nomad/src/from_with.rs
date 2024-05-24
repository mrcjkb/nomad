/// A stateful version of the [`From`] trait.
pub trait FromWith<T, Ctx: ?Sized> {
    /// Turns the value into `Self` using a context.
    fn from_with(value: T, ctx: &Ctx) -> Self;
}

impl<T, Ctx> FromWith<T, Ctx> for T {
    #[inline]
    fn from_with(value: T, _: &Ctx) -> Self {
        value
    }
}

/// A stateful version of the [`Into`] trait.
pub trait IntoWith<T, Ctx: ?Sized> {
    /// Turns `self` into the value using a context.
    fn into_with(self, ctx: &Ctx) -> T;
}

impl<Ctx, T, U> IntoWith<U, Ctx> for T
where
    U: FromWith<T, Ctx>,
{
    #[inline]
    fn into_with(self, ctx: &Ctx) -> U {
        U::from_with(self, ctx)
    }
}
