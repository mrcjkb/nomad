//! TODO: docs.

use core::fmt;

use nvimx_core::{Key, MapAccess, Value, notify};

use crate::oxi::{self, Dictionary, Object, ObjectKind, lua};

/// TODO: docs.
#[derive(Default)]
pub struct NeovimValue {
    object: Object,
}

/// TODO: docs.
pub enum NeovimMapAccess<'a> {
    /// TODO: docs.
    Dict(NeovimDictAccess<'a>),
    /// TODO: docs.
    Nil,
}

/// TODO: docs.
pub struct NeovimDictAccess<'a> {
    dict: &'a mut Dictionary,
    dict_idx: usize,
}

/// TODO: docs.
pub struct NeovimMapPair<'a> {
    dict: &'a mut Dictionary,
    dict_idx: usize,
}

/// TODO: docs.
#[derive(Copy, Clone)]
pub struct NeovimMapKey<'a>(&'a oxi::String);

/// TODO: docs.
pub struct NeovimMapAccessError(ObjectKind);

/// TODO: docs.
pub struct NeovimMapKeyAsStrError;

impl NeovimValue {
    #[inline]
    pub(crate) fn into_inner(self) -> Object {
        self.object
    }

    #[inline]
    pub(crate) fn new(object: Object) -> Self {
        Self { object }
    }
}

impl Value for NeovimValue {
    type MapAccess<'a> = NeovimMapAccess<'a>;
    type MapAccessError<'a> = NeovimMapAccessError;

    #[inline]
    fn map_access(
        &mut self,
    ) -> Result<Self::MapAccess<'_>, Self::MapAccessError<'_>> {
        match self.object.kind() {
            ObjectKind::Dictionary => {
                Ok(NeovimMapAccess::Dict(NeovimDictAccess {
                    // SAFETY: the object's kind is a `Dictionary`.
                    dict: unsafe { self.object.as_dictionary_unchecked_mut() },
                    dict_idx: 0,
                }))
            },
            ObjectKind::Array => {
                // SAFETY: the object's kind is an `Array`.
                let array = unsafe { self.object.as_array_unchecked() };
                if array.is_empty() {
                    Ok(NeovimMapAccess::Nil)
                } else {
                    Err(NeovimMapAccessError(ObjectKind::Array))
                }
            },
            ObjectKind::Nil => Ok(NeovimMapAccess::Nil),
            other => Err(NeovimMapAccessError(other)),
        }
    }
}

impl lua::Poppable for NeovimValue {
    #[inline]
    unsafe fn pop(
        lua_state: *mut lua::ffi::State,
    ) -> Result<Self, lua::Error> {
        unsafe { Object::pop(lua_state).map(|object| Self { object }) }
    }
}

impl lua::Pushable for NeovimValue {
    #[inline]
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        unsafe { self.object.push(lstate) }
    }
}

impl MapAccess for NeovimMapAccess<'_> {
    type Key<'a>
        = NeovimMapKey<'a>
    where
        Self: 'a;
    type Value = NeovimValue;

    #[inline]
    fn next_key(&mut self) -> Option<Self::Key<'_>> {
        match self {
            Self::Dict(dict) => dict.next_key(),
            Self::Nil => None,
        }
    }

    #[inline]
    fn take_next_value(&mut self) -> NeovimValue {
        match self {
            Self::Dict(dict) => dict.take_next_value(),
            Self::Nil => unreachable!(),
        }
    }
}

impl MapAccess for NeovimDictAccess<'_> {
    type Key<'a>
        = NeovimMapKey<'a>
    where
        Self: 'a;
    type Value = NeovimValue;

    #[inline]
    fn next_key(&mut self) -> Option<Self::Key<'_>> {
        if self.dict_idx == self.dict.len() {
            return None;
        }
        todo!();
        // let (key, _) = self.dict.get_by_index(self.dict_idx).unwrap();
        // self.dict_idx += 1;
        // Some(NeovimMapKey(key))
    }

    #[inline]
    fn take_next_value(&mut self) -> NeovimValue {
        // self.dict_idx -= 1;
        // self.dict.swap_remove_by_index(self.dict_idx).unwrap().1
        todo!();
    }
}

impl Key for NeovimMapKey<'_> {
    type AsStrError<'a>
        = NeovimMapKeyAsStrError
    where
        Self: 'a;

    #[inline]
    fn as_str(&self) -> Result<&str, Self::AsStrError<'_>> {
        let Self(key) = self;
        key.to_str().map_err(|_| NeovimMapKeyAsStrError)
    }
}

impl notify::Error for NeovimMapAccessError {
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        Some(notify::Level::Error)
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        todo!()
    }
}

impl fmt::Debug for NeovimMapKey<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl notify::Error for NeovimMapKeyAsStrError {
    #[inline]
    fn to_level(&self) -> Option<notify::Level> {
        Some(notify::Level::Error)
    }

    #[inline]
    fn to_message(&self) -> notify::Message {
        todo!()
    }
}
