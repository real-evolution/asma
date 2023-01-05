use common_macros::proc::parse::{extract_named_fields, extract_struct};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::*, *};

macro_rules! field {
    ($($tt:tt)*) => {
        Field::parse_named.parse2(quote!(pub $($tt)*)).unwrap().into()
    };
}

#[derive(Clone, Copy, Debug, Default, FromMeta)]
pub enum EntityType {
    #[default]
    Mutable,
    Immutable,
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct EntityOptions {
    #[darling(default)]
    pub entity_type: EntityType,
}

pub fn expand_entity(
    args: AttributeArgs,
    mut input: DeriveInput,
) -> TokenStream {
    let args = EntityOptions::from_list(&args).unwrap();
    let type_ident = input.ident.clone();
    let fields = &mut extract_named_fields(extract_struct(&mut input)).named;

    fields.insert(0, field!(id: Key<#type_ident>));
    fields.push(field!(created_at: chrono::DateTime<chrono::Utc>));

    let mut impls: TokenStream = quote! {
        impl Entity for #type_ident{
            fn id(&self) -> &Key<#type_ident> {
                &self.id
            }

            fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
                self.created_at
            }
        }
    };

    if let EntityType::Mutable = args.entity_type {
        fields.push(field!(updated_at: chrono::DateTime<chrono::Utc>));

        impls.extend(quote! {
            impl MutableEntity for #type_ident {
                fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
                    self.updated_at
                }
            }
        });
    }

    quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input

        #impls
    }
}
