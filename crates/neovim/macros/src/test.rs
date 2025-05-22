use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::Nothing;
use syn::{Ident, ItemFn};

#[inline]
pub(crate) fn test(
    attr: proc_macro::TokenStream,
    test_fn: ItemFn,
) -> syn::Result<TokenStream> {
    syn::parse::<Nothing>(attr)?;

    let (
        expand_test,
        maybe_terminator_arg,
        maybe_terminator_name,
        test_output,
    ) = if test_fn.sig.asyncness.is_none() {
        // Sync test.
        (
            Ident::new("sync_test", Span::call_site()),
            None,
            None,
            Some(&test_fn.sig.output),
        )
    } else {
        // Async test.
        let terminator = Ident::new("terminator", Span::call_site());
        (
            Ident::new("async_test", Span::call_site()),
            Some(quote!(#terminator: ::neovim::oxi::tests::TestTerminator)),
            Some(terminator),
            None,
        )
    };

    let test_attrs = &test_fn.attrs;
    let test_name = &test_fn.sig.ident;

    Ok(quote! {
        #[::neovim::oxi::test(nvim_oxi = ::neovim::oxi)]
        #(#test_attrs)*
        fn #test_name(#maybe_terminator_arg) #test_output {
            ::neovim::tests::test_macro::#expand_test(
                {
                    #test_fn
                    #test_name
                },
                ::core::stringify!(#test_name),
            )(#maybe_terminator_name)
        }
    })
}
