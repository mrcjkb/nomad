use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error, LitStr};

struct Hex(LitStr);

impl Parse for Hex {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        input.parse::<LitStr>().map(Self)
    }
}

/// Macro for converting a hex color code to a [`Color`] at compile time.
///
/// # Example
///
/// ```rust
/// # use colorschemes::{hex, Color};
/// assert_eq!(hex!("#ffffff"), Color::new(255, 255, 255));
/// ```
#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    let Hex(color) = parse_macro_input!(input as Hex);

    let value = color.value();

    let mut value = value.as_str();

    if !value.starts_with('#') {
        return Error::new(
            color.span(),
            "hex color codes must start with `#`",
        )
        .into_compile_error()
        .into();
    }

    value = &value[1..];

    if value.len() != 6 {
        return Error::new(
            color.span(),
            "hex color codes must be 6 ascii characters long",
        )
        .into_compile_error()
        .into();
    }

    let (r_str, rest) = value.split_at(2);

    let r = match parse_hex(r_str, color.span()) {
        Ok(r) => r,
        Err(err) => return err,
    };

    let (g_str, b_str) = rest.split_at(2);

    let g = match parse_hex(g_str, color.span()) {
        Ok(g) => g,
        Err(err) => return err,
    };

    let b = match parse_hex(b_str, color.span()) {
        Ok(b) => b,
        Err(err) => return err,
    };

    let expanded = quote! {
        crate::Color::new(#r, #g, #b)
    };

    TokenStream::from(expanded)
}

fn parse_hex(s: &str, span: Span) -> Result<u8, TokenStream> {
    u8::from_str_radix(s, 16)
        .map_err(|err| Error::new(span, err).into_compile_error().into())
}
