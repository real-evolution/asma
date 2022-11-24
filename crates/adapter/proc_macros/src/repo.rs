#![allow(dead_code)]

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(FromDeriveInput)]
#[darling(
    attributes(repo),
    forward_attrs(allow, doc, cfg),
    supports(struct_any)
)]
pub struct RepoDeriveInput {
    ident: syn::Ident,
    data: darling::ast::Data<(), PoolFieldReceiver>,
    table: String,
    read: RepoReadTypes,
}

#[derive(Clone, FromMeta)]
struct RepoReadTypes {
    entity: syn::Type,
    model: syn::Type,
}

#[derive(Debug, FromField)]
#[darling(attributes(pool))]
struct PoolFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl ToTokens for RepoDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let RepoDeriveInput {
            ref ident,
            ref data,
            ref table,
            ref read,
        } = *self;

        let pool = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .first()
            .expect("Should have a pool field as the only field")
            .ident
            .clone()
            .unwrap();

        let (read_entity, read_model) = (&read.entity, &read.model);

        let get_paginated_query = format!(
            r#"
            SELECT * FROM {table}
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2"#,
        );

        tokens.extend(quote! {
            #[async_trait::async_trait]
            impl Repo<#read_entity> for #ident {
                async fn get(&self, id: &Key<#read_entity>) -> RepoResult<#read_entity> {
                    Ok(#read_model::get(self.#pool.get(), id.value())
                        .await
                        .map_err(map_sqlx_error)?
                        .into())
                }

                async fn get_paginated(
                    &self,
                    before: &chrono::DateTime<chrono::Utc>,
                    limit: usize,
                ) -> RepoResult<Vec<#read_entity>> {
                    Ok(sqlx::query_as!(
                        #read_model,
                        #get_paginated_query,
                        before,
                        limit as i64,
                    )
                    .fetch_all(self.#pool.get())
                    .await
                    .map_err(map_sqlx_error)?
                    .into_iter()
                    .map(|u| u.into())
                    .collect())
                }
            }
        });
    }
}

pub fn expand_repo(input: syn::DeriveInput) -> TokenStream {
    let receiver = RepoDeriveInput::from_derive_input(&input).unwrap();

    quote!(#receiver).into()
}
