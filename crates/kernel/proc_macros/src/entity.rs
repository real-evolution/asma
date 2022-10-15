use common_macros::proc::parse::{extract_named_fields, extract_struct};

use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::DeriveInput;
use syn::{parse::*, *};

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
pub enum EntityType {
    Basic,
    Immutable,
    Mutable,
}

impl Default for EntityType {
    fn default() -> Self {
        Self::Mutable
    }
}

impl EntityType {
    fn into_fields(&self, id_type: &Ident) -> Vec<Field> {
        let mut fields = vec![quote!(pub id: #id_type)];

        if self.is_immutable() {
            fields.push(quote!(pub created_at: chrono::DateTime<chrono::Utc>));
        }

        if self.is_mutable() {
            fields.push(quote!(pub updated_at: chrono::DateTime<chrono::Utc>));
        }

        fields
            .into_iter()
            .map(|f| Field::parse_named.parse2(f).unwrap().into())
            .collect()
    }

    fn into_impls(&self, id_type: &Ident, type_ident: &Ident) -> Vec<TokenStream> {
        let mut impls = vec![quote! {
            impl BasicEntity for #type_ident{
                type Key = #id_type;

                fn get_id(&self) -> Self::Key {
                    self.id
                }
            }
        }];

        if self.is_immutable() {
            impls.push(quote! {
                impl ImmutableEntity for #type_ident {
                    fn get_created(&self) -> chrono::DateTime<chrono::Utc> {
                        self.created_at
                    }
                }
            });
        }

        if self.is_mutable() {
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

    fn is_immutable(&self) -> bool {
        match self {
            EntityType::Basic => false,
            _ => true,
        }
    }

    fn is_mutable(&self) -> bool {
        match self {
            EntityType::Mutable => true,
            _ => false,
        }
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

pub fn expand_entity(args: AttributeArgs, mut input: DeriveInput) -> TokenStream {
    let args = EntityOptions::from_list(&args).unwrap();

    let id_inner_type = args
        .id_type
        .unwrap_or(Type::parse.parse2(quote! { uuid::Uuid }).unwrap());
    let id_type = Ident::new(&format!("{}Key", &input.ident), Span::call_site());
    let id_struct = quote! {
        #[derive(Copy, Debug, Clone, sqlx::Type, derive_more::Deref, derive_more::Into)]
        #[sqlx(transparent)]
        pub struct #id_type(#id_inner_type);
    };

    let fields = args.entity_type.into_fields(&id_type);
    let impls = args.entity_type.into_impls(&id_type, &input.ident);

    for field in fields.into_iter().rev() {
        extract_named_fields(extract_struct(&mut input))
            .named
            .insert(0, field);
    }

    quote! {
        #input

        #(#impls)*

        #id_struct
    }
    .into()
}
