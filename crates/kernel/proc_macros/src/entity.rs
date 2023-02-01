use common_macros::proc::parse::{extract_named_fields, extract_struct};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::*, *};

macro_rules! field_bson {
    ($id:ident: $type:ty) => {{
        let field = Field::parse_named
            .parse2(quote! {
                #[serde(
                    serialize_with = "bson::serde_helpers::chrono_datetime_as_bson_datetime::serialize",
                    deserialize_with = "bson::serde_helpers::chrono_datetime_as_bson_datetime::deserialize"
                )]
                pub $id: $type
            })
            .unwrap();

        field
    }};
}
macro_rules! field {
    ($id:ident: $($tt:tt)*) => {
        Field::parse_named.parse2(quote!(pub $id: $($tt)*)).unwrap()
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
    #[darling(default)]
    pub bson_compat: Option<bool>,
}

pub fn expand_entity(
    args: AttributeArgs,
    mut input: DeriveInput,
) -> TokenStream {
    let args = EntityOptions::from_list(&args).unwrap();
    let type_ident = input.ident.clone();
    let fields = &mut extract_named_fields(extract_struct(&mut input)).named;

    fields.insert(0, field!(id: Key<#type_ident>));

    if args.bson_compat.unwrap_or_default() {
        fields.push(field_bson!(created_at: chrono::DateTime<chrono::Utc>));
    } else {
        fields.push(field!(created_at: chrono::DateTime<chrono::Utc>));
    }

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
        if args.bson_compat.unwrap_or_default() {
            fields.push(field_bson!(updated_at: chrono::DateTime<chrono::Utc>));
        } else {
            fields.push(field!(updated_at: chrono::DateTime<chrono::Utc>));
        }

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
