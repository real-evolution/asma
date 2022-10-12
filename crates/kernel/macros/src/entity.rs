use common_macros::proc::parse::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::*, *};

fn add_fields(input: TokenStream, fields: Vec<Field>) -> TokenStream {
    let mut ast = syn::parse2::<DeriveInput>(input).unwrap();

    let struct_data = extract_struct(&mut ast);
    let iter = fields.into_iter();

    match struct_data.fields {
        Fields::Named(ref mut f) => iter.for_each(|i| f.named.push(i)),
        Fields::Unnamed(ref mut f) => iter.for_each(|i| f.unnamed.push(i)),
        Fields::Unit => panic!("`{}` cannot have fields", ast.ident),
    };

    quote! {
        #ast
    }
    .into()
}

fn get_entity_fields<const CREATED: bool, const UPDATED: bool>() -> Vec<Field> {
    let mut fields = vec![quote! { pub id: uuid::Uuid }];

    if CREATED {
        fields.push(quote! { pub created_at: chrono::DateTime<chrono::Utc> });
    }

    if UPDATED {
        fields.push(quote! { pub updated_at: chrono::DateTime<chrono::Utc> });
    }

    fields
        .into_iter()
        .map(|i| Field::parse_named.parse2(i).unwrap())
        .collect()
}

pub fn add_entity_fields<const CREATED: bool, const UPDATED: bool>(
    input: TokenStream,
) -> TokenStream {
    add_fields(input, get_entity_fields::<CREATED, UPDATED>())
}

pub fn implement_entity_trait<const CREATED: bool, const UPDATED: bool>(
    input: TokenStream,
) -> TokenStream {
    let ast = syn::parse2::<DeriveInput>(input).unwrap();
    let type_ident = ast.ident;

    let mut impls = vec![quote! {
        impl Identifiable for #type_ident{
            type Key = uuid::Uuid;

            fn get_id(&self) -> Self::Key {
                self.id
            }
        }
    }];

    if CREATED || UPDATED {
        impls.push(quote! {
            impl Entity for #type_ident {
                fn get_created(&self) -> chrono::DateTime<chrono::Utc> {
                    self.created_at
                }
            }
        });
    }

    if CREATED && UPDATED {
        impls.push(quote! {
            impl MutableEntity for #type_ident {
                fn get_updated(&self) -> chrono::DateTime<chrono::Utc> {
                    self.updated_at
                }
            }
        });
    }

    quote! {
        #(#impls)*
    }
    .into()
}
