use super::parse::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn append_field(input: TokenStream, field: Field) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let data = extract_struct(&mut ast);
    let fields = extract_named_fields(data);

    fields.named.push(field);

    quote! {
        #ast
    }
    .into()
}
