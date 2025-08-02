use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::{calc::AnyExpression, getter::impl_getter};

mod calc;
mod getter;

/// Allow to perform basic `+`, `-`, `*`, `/` between numbers with brackets.
#[proc_macro]
pub fn calc(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as AnyExpression);
    quote! { #c }.into()
}

#[proc_macro_derive(Getter)]
pub fn getter(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_getter(ast)
}
