use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Dollar};
use syn::{
    parse_macro_input,
    parse_quote,
    Block,
    Expr,
    FnArg,
    Ident,
    ItemFn,
    LitInt,
    Pat,
    Signature,
};

#[inline]
pub fn test(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemFn);

    match test_inner(item) {
        Ok(out) => out.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn test_inner(item: ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let ItemFn { sig, block, .. } = item;

    let test_body = test_body(&sig, &block)?;

    let test_name = sig.ident;

    let test_fn_name =
        Ident::new(&format!("__test_fn_{}", test_name), test_name.span());

    let out = quote! {
        #[::nomad::nvim::test(nvim_oxi = ::nomad::nvim, test_fn = #test_fn_name)]
        fn #test_name() {}

        fn #test_fn_name() {
            #test_body
        }
    };

    Ok(out)
}

fn test_body(
    test_sig: &Signature,
    test_body: &Block,
) -> syn::Result<proc_macro2::TokenStream> {
    let seed = Seed::new(&test_sig.inputs)?;

    let define_seed = seed.definition();

    let err_msg = Ident::new("err_msg", Span::call_site());

    let eprintln = if let Seed::None = seed {
        quote! { eprintln!("{}", #err_msg) }
    } else {
        let seed_name = seed.name();
        quote! { eprintln!("failed on seed {}: {}", #seed_name, #err_msg) }
    };

    let into_result = into_result();

    let test_fn = Ident::new("__test_fn", Span::call_site());

    let unwind_body = unwind_body(&seed, &test_fn);

    let inputs = &test_sig.inputs;

    let output = &test_sig.output;

    let out = quote! {
        #into_result

        let panic_msg = ::std::sync::Arc::new(::std::sync::OnceLock::new());

        {
            let panic_msg = panic_msg.clone();

            ::std::panic::set_hook(
                ::std::boxed::Box::new(move |info| {
                    let msg = info
                        .payload()
                        .downcast_ref::<&str>()
                        .map(ToString::to_string)
                        .or_else(|| {
                            info.payload().downcast_ref::<String>().map(Clone::clone)
                        })
                        .unwrap_or_default();
                    let _ = panic_msg.set(msg);
                }),
            );
        }

        fn #test_fn(#inputs) #output {
            #test_body
        }

        #define_seed

        let result = ::std::panic::catch_unwind(|| {
            #unwind_body
        });

        let #err_msg: &dyn ::core::fmt::Display = match &result {
            Ok(Ok(())) => ::std::process::exit(0),
            Ok(Err(err)) => err,
            Err(_panic) => panic_msg.get_or_init(String::new),
        };

        #eprintln;
        ::std::process::exit(1);
    };

    Ok(out)
}

fn unwind_body(seed: &Seed, test_fn: &Ident) -> proc_macro2::TokenStream {
    let generator = if let Seed::None = seed {
        None
    } else {
        Some(Generator { seed_name: seed.name() })
    };

    let mut args = Punctuated::<Expr, Comma>::new();

    if let Some(generator) = &generator {
        let generator = generator.name();
        args.push(parse_quote! { &mut #generator });
    }

    let generator_definition = if let Some(generator) = generator {
        generator.definition()
    } else {
        quote! {}
    };

    quote! {
        #generator_definition
        __IntoResult::into_result(#test_fn(#args))
    }
}

struct Generator {
    seed_name: Ident,
}

impl Generator {
    fn definition(&self) -> proc_macro2::TokenStream {
        let seed = &self.seed_name;
        let this = self.name();
        quote! {
            let mut #this = ::nomad::tests::Generator::new(#seed);
        }
    }

    fn name(&self) -> Ident {
        Ident::new("generator", Span::call_site())
    }
}

enum Seed {
    None,
    RandomlyGenerated,
    Specified(SpecifiedSeed),
}

impl Seed {
    /// Returns the `let seed = ...;` definition.
    fn definition(&self) -> proc_macro2::TokenStream {
        match self {
            Self::None => quote! {},

            Self::RandomlyGenerated => quote! {
                let seed = ::nomad::tests::random_seed();
            },

            Self::Specified(seed) => seed.definition(),
        }
    }

    fn name(&self) -> Ident {
        Ident::new("seed", Span::call_site())
    }

    fn new(inputs: &Punctuated<FnArg, Comma>) -> syn::Result<Self> {
        let Some(first) = inputs.first() else {
            return Ok(Self::None);
        };

        let FnArg::Typed(pat) = first else {
            return Err(syn::Error::new_spanned(
                first,
                "expected a typed argument",
            ));
        };

        let Pat::Ident(pat_ident) = &*pat.pat else {
            return Err(syn::Error::new_spanned(
                pat,
                "expected an identifier",
            ));
        };

        let this = if pat_ident.ident == "gen" {
            Self::RandomlyGenerated
        } else {
            Self::None
        };

        Ok(this)
    }
}

enum SpecifiedSeed {
    Literal(LitInt),
    FromEnv,
}

impl Parse for SpecifiedSeed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitInt) {
            let lit = input.parse()?;
            return Ok(Self::Literal(lit));
        }

        let _ = input.parse::<Dollar>()?;

        let seed = input.parse::<Ident>()?;

        if seed != "SEED" {
            return Err(syn::Error::new_spanned(
                seed,
                "expected `$SEED` or an integer",
            ));
        }

        Ok(Self::FromEnv)
    }
}

impl SpecifiedSeed {
    /// Returns the `let seed = ...;` definition.
    fn definition(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Literal(seed) => {
                quote! {
                    let seed = #seed;
                }
            },

            Self::FromEnv => {
                quote! {
                    let seed = {
                        let Some(env) = ::std::env::var_os("SEED") else {
                            eprintln!("$SEED not set");
                            ::std::process::exit(1);
                        };
                        let Some(str) = env.to_str() else {
                            eprintln!("$SEED is not UTF-8");
                            ::std::process::exit(1);
                        };
                        match str.parse::<u64>() {
                            Ok(seed) => seed,
                            Err(err) => {
                                eprintln!("couldn't parse $SEED: {err}");
                                ::std::process::exit(1);
                            }
                        };
                    };
                }
            },
        }
    }
}

/// Defines the `__IntoResult` trait and implements it for `()` and `Result<(),
/// E>` where `E` is `Debug`.
fn into_result() -> proc_macro2::TokenStream {
    quote! {
        trait __IntoResult {
            type Error: ::core::fmt::Display;
            fn into_result(self) -> ::core::result::Result<(), Self::Error>;
        }
        impl __IntoResult for () {
            type Error = ::core::convert::Infallible;
            fn into_result(self) -> ::core::result::Result<(), Self::Error> {
                Ok(())
            }
        }
        impl<E: ::core::fmt::Display> __IntoResult for ::core::result::Result<(), E> {
            type Error = E;
            fn into_result(self) -> ::core::result::Result<(), E> {
                self
            }
        }
    }
}
