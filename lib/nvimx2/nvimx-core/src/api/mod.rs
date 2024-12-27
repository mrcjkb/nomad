//! TODO: docs.

mod api;
mod api_builder;
mod module_api;
mod module_api_builder;

pub use api::Api;
pub use api_builder::ApiBuilder;
pub use module_api::ModuleApi;
pub use module_api_builder::ModuleApiBuilder;

pub(crate) mod types {
    use super::ModuleApi;
    use crate::{Backend, Module};

    /// Type alias for a `ModuleApi`' s [`Builder`](ModuleApi::Builder).
    pub(crate) type ModuleApiBuilder<'a, M, B> =
        <<<B as Backend>::Api<<M as Module<B>>::Plugin> as super::Api<
            <M as Module<B>>::Plugin,
            B,
        >>::ModuleApi<M> as ModuleApi<M, B>>::Builder<'a>;
}
