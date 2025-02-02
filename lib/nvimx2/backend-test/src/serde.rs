use nvimx_core::notify;
use serde::{Deserialize, Serialize};

use crate::value::TestValue;

/// TODO: docs.
pub struct TestSerializeError {
    inner: serde_json::Error,
}

/// TODO: docs.
pub struct TestDeserializeError {
    inner: serde_json::Error,
}

pub(crate) fn serialize<T>(value: &T) -> Result<TestValue, TestSerializeError>
where
    T: ?Sized + Serialize,
{
    serde_json::to_value(value)
        .map(Into::into)
        .map_err(|inner| TestSerializeError { inner })
}

pub(crate) fn deserialize<'de, T>(
    value: TestValue,
) -> Result<T, TestDeserializeError>
where
    T: Deserialize<'de>,
{
    serde_json::Value::try_from(value)
        .and_then(T::deserialize)
        .map_err(|inner| TestDeserializeError { inner })
}

impl notify::Error for TestSerializeError {
    #[inline]
    fn to_message(&self) -> (notify::Level, notify::Message) {
        (
            notify::Level::Error,
            notify::Message::from_str(self.inner.to_string()),
        )
    }
}

impl notify::Error for TestDeserializeError {
    #[inline]
    fn to_message(&self) -> (notify::Level, notify::Message) {
        (
            notify::Level::Error,
            notify::Message::from_str(self.inner.to_string()),
        )
    }
}
