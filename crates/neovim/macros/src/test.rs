use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::parse::Nothing;
use syn::spanned::Spanned;

#[inline]
pub(crate) fn test(
    attr: proc_macro::TokenStream,
    item: ItemFn,
) -> syn::Result<TokenStream> {
    syn::parse::<Nothing>(attr)?;

    if item.sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            item.sig.ident.span(),
            "expected exactly one argument",
        ));
    }

    let asyncness = &item.sig.asyncness;
    let test_name = &item.sig.ident;
    let test_body = &item.block;
    let test_output = &item.sig.output;

    let ctx_arg = match item.sig.inputs.first().expect("just checked") {
        syn::FnArg::Typed(arg) => arg,
        syn::FnArg::Receiver(_) => {
            return Err(syn::Error::new(
                item.sig.ident.span(),
                "expected a named function argument, not self",
            ));
        },
    };

    let ctx_name = match &*ctx_arg.pat {
        syn::Pat::Ident(pat_ident) => &pat_ident.ident,
        other => {
            return Err(syn::Error::new(
                other.span(),
                "expected a named function argument",
            ));
        },
    };

    let ctx_ty = &ctx_arg.ty;

    let augroup_name = test_name.to_string();

    let run_test = if asyncness.is_some() {
        quote! {
            ::ed::backend::Backend::with_ctx(neovim, |ctx| {
                ctx.spawn_local_unprotected(inner)
            })
            .await
        }
    } else {
        quote! {
            ::ed::backend::Backend::with_ctx(neovim, inner)
        }
    };

    Ok(quote! {
        #[::neovim::oxi::test(nvim_oxi = ::neovim::oxi)]
        #asyncness fn #test_name() #test_output {
            #[inline]
            #asyncness fn inner(#ctx_name: #ctx_ty) #test_output {
                #test_body
            }
            let neovim = ::neovim::Neovim::init(#augroup_name);
            #run_test
        }
    })
}
