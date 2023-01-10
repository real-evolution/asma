use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use kernel_entities::traits::Key;
use kernel_repositories::{
    error::{RepoError, RepoResult},
    traits::Repo,
};
use mongodb::{
    bson::{doc, Document},
    options::{FindOneOptions, FindOptions, IndexOptions},
    Collection, Database,
};
use tokio_stream::StreamExt;
use tracing::debug;

use crate::{
    traits::collection_entity::CollectionEntity,
    util::{error::map_mongo_error, index},
};

const ENTITY_ID_FIELD: &str = "id";
const ENTITY_CREATED_AT_FIELD: &str = "created_at";

pub struct MongoDbRepo<E> {
    database: Database,
    _phantom: PhantomData<E>,
}

impl<E: CollectionEntity> MongoDbRepo<E> {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            _phantom: PhantomData,
        }
    }

    pub async fn initialize(&self) -> RepoResult<()> {
        debug!("initializing `{}` documents repo", E::name());

        let collection = self.collection();

        // `id` index
        index::create_index(
            &collection,
            doc! {ENTITY_ID_FIELD: 1},
            Some(IndexOptions::builder().unique(Some(true)).build()),
        )
        .await?;

        // `created_at` index
        index::create_index(
            &collection,
            doc! {ENTITY_CREATED_AT_FIELD: -1},
            None,
        )
        .await?;

        // custom initializer
        E::initialize_collection(&collection).await
    }

    pub async fn find_stream<D, O>(
        &self,
        filter: D,
        options: O,
    ) -> RepoResult<impl tokio_stream::Stream<Item = RepoResult<E>>>
    where
        D: Into<Option<Document>>,
        O: Into<Option<FindOptions>>,
    {
        let stream = self
            .collection()
            .find(filter, options)
            .await
            .map_err(map_mongo_error)?
            .map_err(map_mongo_error);

        Ok(stream)
    }

    pub async fn find_one<'a, D, O>(
        &self,
        filter: D,
        options: O,
    ) -> RepoResult<E>
    where
        D: Into<Option<Document>>,
        O: Into<Option<FindOneOptions>>,
    {
        self.collection()
            .find_one(filter, options)
            .await
            .map_err(map_mongo_error)?
            .ok_or(RepoError::NotFound)
    }

    pub fn collection(&self) -> Collection<E> {
        self.database.collection(E::name())
    }
}

#[async_trait::async_trait]
impl<E: CollectionEntity> Repo for MongoDbRepo<E> {
    type Entity = E;

    async fn get(&self, key: &Key<Self::Entity>) -> RepoResult<Self::Entity> {
        self.find_one(doc! {ENTITY_ID_FIELD: key.to_string()}, None)
            .await
    }

    async fn get_paginated(
        &self,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        self.find_stream(
            doc! {ENTITY_CREATED_AT_FIELD: {"$lt": before}},
            FindOptions::builder()
                .sort(doc! { ENTITY_CREATED_AT_FIELD: -1})
                .build(),
        )
        .await?
        .take(limit)
        .collect()
        .await
    }

    async fn exists(&self, key: &Key<Self::Entity>) -> RepoResult<bool> {
        let count = self
            .collection()
            .count_documents(doc! { ENTITY_ID_FIELD: key.to_string()}, None)
            .await
            .unwrap_or(0);

        assert!(count <= 1);

        Ok(count > 0)
    }

    async fn remove(&self, key: &Key<Self::Entity>) -> RepoResult<()> {
        let ret = self
            .collection()
            .delete_one(doc! { ENTITY_ID_FIELD: key.to_string()}, None)
            .await
            .map_err(map_mongo_error)?;

        if ret.deleted_count != 1 {
            return Err(RepoError::NotFound);
        }

        Ok(())
    }
}
