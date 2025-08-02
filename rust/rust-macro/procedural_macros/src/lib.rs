use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::calc::AnyExpression;

mod calc;

/// Allow to perform basic `+`, `-`, `*`, `/` between numbers with brackets.
#[proc_macro]
pub fn calc(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as AnyExpression);
    quote! { #c }.into()
}
