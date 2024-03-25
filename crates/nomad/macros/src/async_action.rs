use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Error, ItemImpl};

pub fn async_action(mut item: ItemImpl) -> Result<TokenStream, Error> {
    let execute = item.items.iter_mut().find_map(|item| match item {
        syn::ImplItem::Fn(func) if func.sig.ident == "execute" => Some(func),
        _ => None,
    });

    let Some(execute) = execute else {
        // There is no `execute` method in the impl block. This'll already
        // result in a compile error, so we don't have do nothing here.
        return Ok(quote!(#item));
    };

    if execute.sig.asyncness.is_none() {
        return Err(Error::new_spanned(&execute.sig, ExecuteNotAsyncError));
    }

    transform_signature(&mut execute.sig);

    transform_block(&mut execute.block);

    Ok(quote!(#item))
}

fn transform_signature(old: &mut syn::Signature) {
    old.asyncness = None;
    transform_return_type(&mut old.output);
}

fn transform_block(old: &mut syn::Block) {
    let expr: syn::Expr = parse_quote! {
        ::nomad::maybe_future::MaybeFutureEnum::from(async move { #old })
    };

    let stmt = syn::Stmt::Expr(expr, None);

    old.stmts = vec![stmt];
}

fn transform_return_type(old: &mut syn::ReturnType) {
    let ty = loop {
        match old {
            syn::ReturnType::Default => {
                *old = syn::ReturnType::Type(
                    Default::default(),
                    Box::new(unit_type()),
                );
            },

            syn::ReturnType::Type(_, ty) => break &mut **ty,
        }
    };

    let bound = syn::TypeParamBound::Verbatim(
        quote!(::nomad::maybe_future::MaybeFuture<Output = #ty>),
    );

    let new_ty = syn::Type::ImplTrait(syn::TypeImplTrait {
        impl_token: syn::token::Impl::default(),
        bounds: core::iter::once(bound).collect(),
    });

    *ty = new_ty;
}

fn unit_type() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    })
}

struct ExecuteNotAsyncError;

impl core::fmt::Display for ExecuteNotAsyncError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "the `execute` method must be async")
    }
}
