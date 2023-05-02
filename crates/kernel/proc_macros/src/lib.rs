mod entity;

use darling::{export::NestedMeta, Error};
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let args = match NestedMeta::parse_meta_list(args.into()) {
        | Ok(v) => v,
        | Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    entity::expand_entity(args, input).into()
}
