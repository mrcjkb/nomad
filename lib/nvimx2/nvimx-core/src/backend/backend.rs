//! TODO: docs.

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::backend::{Api, BackgroundExecutor, LocalExecutor, Value};
use crate::notify::{self, MaybeResult};
use crate::plugin::Plugin;

/// TODO: docs.
pub trait Backend: 'static + Sized {
    /// TODO: docs.
    type Api<P: Plugin<Self>>: Api<P, Self>;

    /// TODO: docs.
    type ApiValue: Value<Self>;

    /// TODO: docs.
    type LocalExecutor: LocalExecutor;

    /// TODO: docs.
    type BackgroundExecutor: BackgroundExecutor;

    /// TODO: docs.
    type Emitter<'this>: notify::Emitter;

    /// TODO: docs.
    fn api<P: Plugin<Self>>(&mut self) -> Self::Api<P>;

    /// TODO: docs.
    fn init() -> Self;

    /// TODO: docs.
    fn emitter(&mut self) -> Self::Emitter<'_>;

    /// TODO: docs.
    fn local_executor(&mut self) -> &mut Self::LocalExecutor;

    /// TODO: docs.
    fn background_executor(&mut self) -> &mut Self::BackgroundExecutor;

    /// TODO: docs.
    fn serialize<T>(
        &mut self,
        value: &T,
    ) -> impl MaybeResult<Self::ApiValue, Self>
    where
        T: ?Sized + Serialize;

    /// TODO: docs.
    fn deserialize<T>(
        &mut self,
        value: Self::ApiValue,
    ) -> impl MaybeResult<T, Self>
    where
        T: DeserializeOwned;
}
