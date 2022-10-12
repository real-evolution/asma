use common_macros::proc::parse::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, *};

pub fn add_entity_fields(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_data = extract_struct(&mut ast);
    let named_fields = extract_named_fields(struct_data);

    let fields = vec![
        quote! { pub id: uuid::Uuid },
        quote! { pub created_at: chrono::DateTime<chrono::Utc> },
        quote! { pub updated_at: chrono::DateTime<chrono::Utc> },
    ]
    .into_iter()
    .map(|i| Field::parse_named.parse2(i).unwrap());

    for field in fields {
        named_fields.named.push(field);
    }

    quote! {
        #ast
    }
    .into()
}

pub fn implement_entity_trait(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_ident = ast.ident;

    quote! {
        impl Entity<uuid::Uuid> for #type_ident {
            fn get_id(&self) -> uuid::Uuid {
                self.id
            }

            fn get_created(&self) -> chrono::DateTime<chrono::Utc> {
                self.created_at
            }

            fn get_updated(&self) -> chrono::DateTime<chrono::Utc> {
                self.updated_at
            }
        }
    }
    .into()
}
