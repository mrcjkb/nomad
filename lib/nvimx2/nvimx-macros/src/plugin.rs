use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Type};

#[inline]
pub(crate) fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let version = parse_macro_input!(attr as Type);
    let fun = parse_macro_input!(item as ItemFn);
    let fun_name = &fun.sig.ident;
    let fun_body = &fun.block;

    quote! {
        #[::nvimx2::neovim::oxi::plugin(nvim_oxi = ::nvimx2::neovim::oxi)]
        fn #fun_name() -> ::nvimx2::neovim::oxi::Dictionary {
            let mut __backend: ::nvimx2::neovim::Neovim<#version> = ::nvimx2::Backend::init();
            let __plugin_ctx = ::nvimx2::PluginCtx::new(&mut __backend);
            let __plugin = #fun_body;
            let __plugin_api = ::nvimx2::Plugin::api(&__plugin, __plugin_ctx);
            ::nvimx2::Backend::to_backend_api(&mut __backend, __plugin_api)
        }
    }
    .into()
}
