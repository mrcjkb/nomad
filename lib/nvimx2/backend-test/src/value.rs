use indexmap::IndexMap;
use nvimx_core::backend::{MapAccess, Value};
use nvimx_core::notify;

use crate::TestBackend;

/// TODO: docs.
pub enum TestValue {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    List(Vec<Self>),
    Map(TestMap),
    Function(Box<dyn FnMut(Self) -> Self>),
}

pub struct TestMap {
    inner: IndexMap<String, TestValue>,
}

pub struct TestMapAccessError {
    kind: &'static str,
}

impl TestValue {
    fn kind(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "boolean",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::List(_) => "list",
            Self::Map(_) => "map",
            Self::Function(_) => "function",
        }
    }
}

impl Value<TestBackend> for TestValue {
    type MapAccess<'a> = (&'a mut TestMap, Option<usize>);
    type MapAccessError<'a> = TestMapAccessError;

    #[inline]
    fn map_access(
        &mut self,
    ) -> Result<Self::MapAccess<'_>, Self::MapAccessError<'_>> {
        match self {
            Self::Map(map) => Ok((map, None)),
            _ => Err(TestMapAccessError { kind: self.kind() }),
        }
    }
}

impl MapAccess<TestBackend> for (&mut TestMap, Option<usize>) {
    type Key<'a>
        = &'a str
    where
        Self: 'a;
    type Value = TestValue;

    fn next_key(&mut self) -> Option<Self::Key<'_>> {
        let (map, maybe_idx) = self;
        let mut is_first_access = false;
        let idx = maybe_idx.get_or_insert_with(|| {
            is_first_access = true;
            0
        });
        let maybe_key = map.inner.get_index(*idx).map(|(key, _)| &**key);
        *idx += !is_first_access as usize;
        maybe_key
    }

    fn take_next_value(&mut self) -> Self::Value {
        let (map, maybe_idx) = self;
        let idx = maybe_idx.expect("already called next_key");
        let (_, value) = map.inner.swap_remove_index(idx).expect("not oob");
        value
    }
}

impl notify::Error for TestMapAccessError {
    #[inline]
    fn to_message(&self) -> (notify::Level, notify::Message) {
        let msg = format!("expected a map, got {} instead", self.kind);
        (notify::Level::Error, notify::Message::from_str(msg))
    }
}
