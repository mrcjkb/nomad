use std::hash::{Hash, Hasher};

use crate::maybe_result::MaybeResult;
use crate::Module;

/// The output of calling [`as_str`](ActionName::as_str) on an [`ActionName`].
pub(crate) type ActionNameStr = &'static str;

/// TODO: docs
pub trait Action: 'static {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args;

    /// TODO: docs
    type Docs;

    /// TODO: docs
    type Module: Module;

    /// TODO: docs
    //
    // NOTE: remove once we have RTN
    // (https://github.com/rust-lang/rust/issues/109417).
    type Return;

    /// TODO: docs
    fn execute(&mut self, args: Self::Args) -> impl MaybeResult<Self::Return>;

    /// TODO: docs
    fn docs(&self) -> Self::Docs;
}

/// TODO: docs
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionName {
    name: &'static str,
}

impl core::fmt::Debug for ActionName {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActionName").field(&self.name).finish()
    }
}

impl core::fmt::Display for ActionName {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name)
    }
}

impl AsRef<str> for ActionName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name
    }
}

impl ActionName {
    /// TODO: docs
    #[inline]
    pub(crate) fn as_str(&self) -> ActionNameStr {
        self.name
    }

    #[doc(hidden)]
    pub const fn from_str(name: ActionNameStr) -> Self {
        Self { name }
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn id(&self) -> ActionId {
        ActionId::from_action_name(self.name)
    }
}

/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ActionId(u64);

impl ActionId {
    /// TODO: docs
    #[inline]
    pub(crate) fn from_action_name(name: &str) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();
        Self(hash)
    }
}
