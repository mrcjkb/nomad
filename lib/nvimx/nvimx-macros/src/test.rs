use core::fmt;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Block, FnArg, Ident, ItemFn, Path, parse_quote};

pub(crate) fn test(args: Args, item: ItemFn) -> syn::Result<TokenStream> {
    let test = Test::new(&args, &item)?;
    let test_body = test.body();
    let maybe_terminator = test.terminator();

    let nvimx = &args.nvimx.path();

    let test_name = &item.sig.ident;
    let test_output = &item.sig.output;
    let test_attrs = item
        .attrs
        .iter()
        .map(ToTokens::into_token_stream)
        .collect::<proc_macro2::TokenStream>();

    Ok(quote! {
        #test_attrs
        #[#nvimx::oxi::test(
            nvim_oxi = #nvimx::oxi,
            library_path = #nvimx::tests::test_macro::library_path(env!("CARGO_CRATE_NAME")),
        )]
        fn #test_name(#maybe_terminator) #test_output {
            #test_body
        }
    })
}

struct Test<'a> {
    nvimx_path: NvimxPathArg,
    orig: &'a ItemFn,
    terminator_name: Option<Ident>,
}

impl<'a> Test<'a> {
    #[inline]
    fn body(&self) -> Block {
        let Some(terminator) = self.terminator_name.as_ref() else {
            return self.orig.block.as_ref().clone();
        };

        let orig_body = &self.orig.block;
        let nvimx = &self.nvimx_path.path();

        parse_quote! {{
            #nvimx::tests::test_macro::run_async_test(#terminator, async move {
                (async #orig_body).await
            })
        }}
    }

    #[inline]
    fn new(args: &Args, orig: &'a ItemFn) -> syn::Result<Self> {
        if !orig.sig.inputs.is_empty() {
            return Err(syn::Error::new_spanned(
                &orig.sig.inputs,
                TestFunctionHasArgsError,
            ));
        }

        Ok(Self {
            nvimx_path: args.nvimx.clone(),
            orig,
            terminator_name: orig
                .sig
                .asyncness
                .map(|_| Ident::new("__test_terminator", Span::call_site())),
        })
    }

    #[inline]
    fn terminator(&self) -> Option<FnArg> {
        self.terminator_name.as_ref().map(|terminator| {
            let nvimx = &self.nvimx_path.path();
            parse_quote! {
                #terminator: #nvimx::oxi::TestTerminator
            }
        })
    }
}

/// The `..` in `#[nvimx::macros::test(..)]`.
#[derive(syn_derive_args::Parse)]
#[args(default)]
pub(crate) struct Args {
    nvimx: NvimxPathArg,
}

/// A argument given to `test` used to specify the path to the `nvimx` crate.
#[derive(Clone)]
struct NvimxPathArg(Path);

impl NvimxPathArg {
    #[inline]
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Default for NvimxPathArg {
    #[inline]
    fn default() -> Self {
        Self(parse_quote!(::nvimx))
    }
}

impl syn::parse::Parse for NvimxPathArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

/// An error that's returned when the function annotated with `#[test]` has
/// arguments.
struct TestFunctionHasArgsError;

impl fmt::Display for TestFunctionHasArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "functions annotated with `test` must not have arguments")
    }
}
