//! TODO: docs.

mod buf_add;

pub use buf_add::BufAdd;

/// TODO: docs.
pub trait Event: Sized {
    /// TODO: docs.
    type Ctx<'a>;

    /// TODO: docs.
    fn register(self, ctx: Self::Ctx<'_>);
}
