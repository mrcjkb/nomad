use core::fmt;

use smallvec::{SmallVec, smallvec};
use smol_str::SmolStrBuilder;

use crate::notify::Name;

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    names: SmallVec<[Name; 2]>,
}

impl Namespace {
    /// TODO: docs.
    #[inline]
    pub fn dot_separated(&self) -> impl fmt::Display {
        let mut builder = SmolStrBuilder::new();
        let mut names = self.names();
        if let Some(first) = names.next() {
            builder.push_str(first);
            for name in names {
                builder.push('.');
                builder.push_str(name);
            }
        }
        builder.finish()
    }

    /// TODO: docs.
    #[inline]
    pub fn names(&self) -> impl ExactSizeIterator<Item = Name> + '_ {
        self.names.iter().copied()
    }

    /// TODO: docs.
    #[inline]
    pub fn plugin_name(&self) -> Name {
        self.names[0]
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(plugin_name: Name) -> Self {
        Self { names: smallvec![plugin_name] }
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn push(&mut self, module_name: Name) {
        self.names.push(module_name);
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn pop(&mut self) {
        self.names.pop();
    }
}

impl FromIterator<Name> for Namespace {
    #[track_caller]
    fn from_iter<T: IntoIterator<Item = Name>>(iter: T) -> Self {
        let names = iter.into_iter().collect::<SmallVec<_>>();
        if names.is_empty() {
            panic!(
                "a Namespace must have at least one name representing the \
                 plugin name"
            );
        }
        Self { names }
    }
}
