//! TODO: docs.

use core::fmt;

use nvimx_core::{Key, KeyValuePair, MapAccess, Value, notify};

use crate::oxi::{self, Dictionary, Object, ObjectKind, lua};

/// TODO: docs.
#[derive(Default)]
pub struct NeovimValue {
    object: Object,
}

/// TODO: docs.
pub struct NeovimMapAccess<'a> {
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
pub struct NeovimMapKey<'a> {
    dict_key: &'a oxi::String,
    dict_idx: usize,
}

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
            ObjectKind::Dictionary => Ok(NeovimMapAccess {
                // SAFETY: the object's kind is a `Dictionary`.
                dict: unsafe { self.object.as_dictionary_unchecked_mut() },
                dict_idx: 0,
            }),
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

impl<'a> MapAccess for NeovimMapAccess<'a> {
    type Pair = NeovimMapPair<'a>;

    #[inline]
    fn next_pair(&mut self) -> Option<Self::Pair> {
        todo!()
    }
}

impl KeyValuePair for NeovimMapPair<'_> {
    type Key<'a>
        = NeovimMapKey<'a>
    where
        Self: 'a;

    type Value = NeovimValue;

    #[inline]
    fn key(&self) -> Self::Key<'_> {
        // let (dict_key, _) = self.dict.get_by_index(self.dict_idx).unwrap();
        // NeovimMapKey { dict_key, dict_idx: self.dict_idx }
        todo!();
    }

    #[inline]
    fn take_value(self) -> Self::Value {
        // let idx = self.dict_idx;
        // let (_, value) = self.dict.swap_remove_by_index(idx).unwrap();
        // value
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
        todo!()
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
        todo!()
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
