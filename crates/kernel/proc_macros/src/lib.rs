mod entity;
mod service;

use proc_macro::TokenStream;
use syn::{AttributeArgs, DeriveInput};

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);
    let input = syn::parse_macro_input!(input as DeriveInput);

    entity::expand_entity(args, input).into()
}

#[proc_macro_derive(Service)]
pub fn service(input: TokenStream) -> TokenStream {
    service::expand_service(input.into()).into()
}
