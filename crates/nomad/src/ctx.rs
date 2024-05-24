use std::fmt;

use crate::Shared;

/// TODO: docs.
#[derive(Clone, Default)]
pub struct Ctx {
    _inner: Shared<CtxInner>,
}

impl Ctx {}

#[derive(Default)]
struct CtxInner {}

impl fmt::Debug for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Ctx")
    }
}
