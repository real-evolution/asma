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
    #[darling(default)]
    insert: Option<RepoInsertTypes>,
}

#[derive(Clone, FromMeta)]
struct RepoReadTypes {
    entity: syn::Type,
    model: syn::Type,
}

#[derive(Clone, FromMeta)]
struct RepoInsertTypes {
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
            ref insert,
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

                async fn remove(&self, key: &Key<#read_entity>) -> RepoResult<()> {
                    #read_model::delete_row(self.db.get(), key.value())
                        .await
                        .map_err(map_sqlx_error)?;

                    Ok(())
                }
            }
        });

        if let Some(insert) = insert {
            let (insert_entity, insert_model) = (&insert.entity, &insert.model);

            tokens.extend(quote! {
                #[async_trait::async_trait]
                impl InsertRepo<#read_entity, #insert_entity> for #ident {
                    async fn create(&self, insert: #insert_entity) -> RepoResult<#read_entity> {
                        let insert: #insert_model = insert.into();

                        Ok(
                            #read_model::insert(
                                self.db.acquire().await?.as_mut(),
                                insert,
                            )
                            .await
                            .map_err(map_sqlx_error)?
                            .into(),
                        )
                    }
                }
            });
        }
    }
}

pub fn expand_repo(input: syn::DeriveInput) -> TokenStream {
    let receiver = RepoDeriveInput::from_derive_input(&input).unwrap();

    quote!(#receiver).into()
}
