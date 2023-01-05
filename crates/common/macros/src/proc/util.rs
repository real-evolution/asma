use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

use super::parse::*;

pub fn append_fields(input: TokenStream, fields: Vec<Field>) -> TokenStream {
    let mut ast = syn::parse2::<DeriveInput>(input).unwrap();

    let struct_data = extract_struct(&mut ast);
    let iter = fields.into_iter();

    match struct_data.fields {
        | Fields::Named(ref mut f) => iter.for_each(|i| f.named.push(i)),
        | Fields::Unnamed(ref mut f) => iter.for_each(|i| f.unnamed.push(i)),
        | Fields::Unit => panic!("`{}` cannot have fields", ast.ident),
    };

    quote!( #ast )
}
