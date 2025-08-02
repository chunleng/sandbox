use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Expr, Field, Fields, parse_str};

pub fn impl_getter(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    let fields = match ast.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(f) => f.named,
            _ => {
                panic!("This should be unreachable");
            }
        },
        _ => {
            panic!("Getter can only be used on structs");
        }
    }
    .into_iter()
    .map(|x| gen_getter_func(x));

    quote! {
        impl #ident {
            #(#fields)*
        }
    }
    .into()
}

fn gen_getter_func(field: Field) -> TokenStream2 {
    match field.ident {
        Some(field_id) => {
            let func_name: Expr = parse_str(&format!("get_{}", field_id)).expect(&format!(
                "Problem converting {} into function name",
                field_id
            ));
            let field_type = field.ty;
            quote! {
                fn #func_name(&self) -> #field_type {
                    self.#field_id.to_owned()
                }
            }
        }
        None => {
            quote! {}
        }
    }
    .to_token_stream()
}
