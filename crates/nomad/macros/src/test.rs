use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, ItemFn};

#[inline]
pub fn test(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { sig, block, .. } = parse_macro_input!(item as syn::ItemFn);

    let test_name = sig.ident;

    let test_body = test_body(&block);

    quote! {
        #[::nomad::nvim::test(nvim_oxi = ::nomad::nvim)]
        fn #test_name() -> ::std::thread::Result<()> {
            #test_body
        }
    }
    .into()
}

fn test_body(test_body: &Block) -> proc_macro2::TokenStream {
    quote! {
        ::std::panic::catch_unwind(|| {
            let __seed = ::nomad::tests::random_seed();
            let mut gen = ::nomad::tests::Generator::new(__seed);
            let gen = &mut gen;
            #test_body
        })
    }
}
