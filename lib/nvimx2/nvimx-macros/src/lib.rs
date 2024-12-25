//! TODO: docs.

#[cfg(feature = "plugin")]
mod plugin;

/// TODO: docs.
#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    plugin::plugin(attr, item)
}
