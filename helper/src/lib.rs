use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, Visibility, parse_macro_input, token::Pub};

#[proc_macro_attribute]
pub fn pub_fields(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    match input.fields {
        Fields::Named(ref mut fields) => {
            for field in fields.named.iter_mut() {
                field.vis = Visibility::Public(Pub::default());
            }
        }
        Fields::Unnamed(ref mut fields) => {
            for field in fields.unnamed.iter_mut() {
                field.vis = Visibility::Public(Pub::default());
            }
        }
        _ => {}
    }
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}
