use common_macros::proc::parse::{extract_named_fields, extract_struct};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::DeriveInput;
use syn::{parse::*, *};

macro_rules! field {
    ($name:ident, $typ:ty) => {
        Field::parse_named.parse2(quote!(pub $name: $typ)).unwrap().into()
    };

    ($name:ident, #$typ:tt) => {
        Field::parse_named.parse2(quote!(pub $name: #$typ)).unwrap().into()
    };
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
pub enum EntityType {
    Immutable,
    Mutable,
}

impl Default for EntityType {
    fn default() -> Self {
        Self::Mutable
    }
}

impl EntityType {
    fn into_impls(
        &self,
        id_type: &Ident,
        type_ident: &Ident,
    ) -> Vec<TokenStream> {
        let mut impls = vec![quote! {
            impl BasicEntity for #type_ident{
                type Key = #id_type;

                fn get_id(&self) -> Self::Key {
                    self.id
                }

                fn get_created(&self) -> chrono::DateTime<chrono::Utc> {
                    self.created_at
                }
            }
        }];

        if let EntityType::Mutable = self {
            impls.push(quote! {
                impl MutableEntity for #type_ident {
                    fn get_updated(&self) -> chrono::DateTime<chrono::Utc> {
                        self.updated_at
                    }
                }
            });
        }

        impls
    }
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct EntityOptions {
    #[darling(default)]
    pub id_type: Option<syn::Type>,
    #[darling(default)]
    pub entity_type: EntityType,
}

pub fn expand_entity(
    args: AttributeArgs,
    mut input: DeriveInput,
) -> TokenStream {
    let args = EntityOptions::from_list(&args).unwrap();

    let id_type =
        Ident::new(&format!("{}Key", &input.ident), Span::call_site());
    let id_inner_type = args
        .id_type
        .unwrap_or(Type::parse.parse2(quote! { uuid::Uuid }).unwrap());

    let fields = &mut extract_named_fields(extract_struct(&mut input)).named;

    fields.insert(0, field!(id, #id_type));
    fields.push(field!(created_at, chrono::DateTime<chrono::Utc>));

    if let EntityType::Mutable = args.entity_type {
        fields.push(field!(updated_at, chrono::DateTime<chrono::Utc>));
    }

    let impls = args.entity_type.into_impls(&id_type, &input.ident);

    quote! {
        #[derive(serde::Deserialize, serde::Serialize)]
        #input

        #(#impls)*

        #[derive(Clone,
                 Copy,
                 Debug,
                 serde::Serialize,
                 serde::Deserialize,
                 sqlx::Type,
                 derive_more::Into,
                 derive_more::From,
                 derive_more::Display)]
        #[sqlx(transparent)]
        pub struct #id_type(pub #id_inner_type);
    }
    .into()
}
