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

impl ToTokens for RepoDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let RepoDeriveInput {
            ref ident,
            ref table,
            ref read,
            ref insert,
        } = *self;

        let (read_entity, read_model) = (&read.entity, &read.model);

        let get_paginated_query = format!(
            r#"
            SELECT * FROM {table}
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2"#,
        );

        let exists_query =
            format!(r#"SELECT EXISTS (SELECT 1 FROM {table} WHERE id = $1)"#,);

        tokens.extend(quote! {
            #[async_trait::async_trait]
            impl Repo for #ident {
                type Entity = #read_entity;

                async fn get(&self, id: &Key<#read_entity>) -> RepoResult<#read_entity> {
                    Ok(#read_model::get(self.0.get(), id.value())
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
                    .fetch_all(self.0.get())
                    .await
                    .map_err(map_sqlx_error)?
                    .into_iter()
                    .map(|u| u.into())
                    .collect())
                }

                async fn exists(&self, key: &Key<#read_entity>) -> RepoResult<bool> {
                    let exists = sqlx::query_scalar!(
                        #exists_query,
                        key.value_ref(),
                    )
                    .fetch_one(self.0.get())
                    .await
                    .map_err(map_sqlx_error)?;

                    Ok(exists.unwrap_or(false))
                }

                async fn remove(&self, key: &Key<#read_entity>) -> RepoResult<()> {
                    #read_model::delete_row(self.0.get(), key.value())
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
                impl InsertRepo<#insert_entity> for #ident {
                    async fn create(&self, insert: #insert_entity) -> RepoResult<Self::Entity> {
                        let insert: #insert_model = insert.into();

                        Ok(
                            #read_model::insert(
                                self.0.acquire().await?.as_mut(),
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

    quote!(#receiver)
}
