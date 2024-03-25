use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn ready(input: DeriveInput) -> TokenStream {
    let symbol_name = &input.ident;

    quote! {
        impl<'a> ::nomad::maybe_future::MaybeFuture<'a> for #symbol_name {
            type Output = Self;

            fn into_enum(self) ->::nomad::maybe_future::MaybeFutureEnum<'a, #symbol_name> {
                ::nomad::maybe_future::MaybeFutureEnum::Ready(self)
            }
        }
    }
}
