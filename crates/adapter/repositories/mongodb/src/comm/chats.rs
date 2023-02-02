use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, ChatState, Message},
    },
    traits::Key,
};
use kernel_repositories::{
    comm::{ChatsRepo, InsertChat},
    error::{RepoError, RepoResult},
    traits::{ChildRepo, InsertRepo, StatsPair, StatsRepo},
};
use mongodb::{
    bson::{doc, Document},
    options::{ChangeStreamOptions, FindOptions, FullDocumentType},
};
use tokio_stream::StreamExt;

use crate::{
    repo::{MongoDbRepo, ENTITY_CREATED_AT_FIELD, ENTITY_ID_FIELD},
    traits::collection_entity::CollectionEntity,
    util::error::map_mongo_error,
};

#[async_trait::async_trait]
impl ChatsRepo for MongoDbRepo<Chat> {
    async fn watch(
        &self,
        id: &Key<Chat>,
    ) -> RepoResult<BoxStream<'_, RepoResult<Message>>> {
        let filter = doc! {
            "$match": {
                "$and": [
                    { "fullDocument.chat_id": id.value_ref() },
                    { "operationType": "insert" }
                ]
            }
        };

        self.watch_messages(filter).await
    }

    async fn watch_all_of(
        &self,
        user_id: &Key<User>,
    ) -> RepoResult<BoxStream<'static, RepoResult<Message>>> {
        let filter = doc! {
            "$match": {
                "$and": [
                    { "fullDocument.user_id": user_id.value_ref() },
                    { "operationType": "insert" }
                ]
            }
        };

        self.watch_messages(filter).await
    }
}

#[async_trait::async_trait]
impl InsertRepo<InsertChat> for MongoDbRepo<Chat> {
    async fn create(&self, model: InsertChat) -> RepoResult<Self::Entity> {
        let chat = Chat {
            id: uuid::Uuid::new_v4().into(),
            label: model.label,
            state: model.state,
            user_id: model.user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.collection()
            .insert_one(&chat, None)
            .await
            .map_err(map_mongo_error)?;

        Ok(chat)
    }
}

#[async_trait::async_trait]
impl ChildRepo<User> for MongoDbRepo<Chat> {
    async fn get_paginated_of(
        &self,
        parent_key: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        self.find_stream(
            doc! {
                ENTITY_CREATED_AT_FIELD: {"$lt": before},
                "user_id": parent_key.value_ref()
            },
            FindOptions::builder()
                .sort(doc! { ENTITY_CREATED_AT_FIELD: -1})
                .build(),
        )
        .await?
        .take(limit)
        .collect()
        .await
    }

    async fn get_of(
        &self,
        parent_key: &Key<User>,
        key: &Key<Self::Entity>,
    ) -> RepoResult<Self::Entity> {
        self.find_one(doc! {ENTITY_ID_FIELD: key.value_ref(), "user_id": parent_key.value_ref()}, None)
            .await
    }

    async fn remove_of(
        &self,
        parent_key: &Key<User>,
        key: &Key<Self::Entity>,
    ) -> RepoResult<()> {
        let ret = self
            .collection()
            .delete_one(doc! { ENTITY_ID_FIELD: key.value_ref(), "user_id": parent_key.value_ref()}, None)
            .await
            .map_err(map_mongo_error)?;

        if ret.deleted_count != 1 {
            return Err(RepoError::NotFound);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl StatsRepo<User> for MongoDbRepo<Chat> {
    async fn get_stats_for(
        &self,
        user_id: &Key<User>,
    ) -> RepoResult<StatsPair> {
        let total = self
            .collection()
            .count_documents(None, None)
            .await
            .map_err(map_mongo_error)?;

        let active = self
            .collection()
            .count_documents(
                doc! {
                    "user_id": user_id.value_ref(),
                    "state": ChatState::Active.to_string()
                },
                None,
            )
            .await
            .map_err(map_mongo_error)?;

        Ok(StatsPair::new(total, active))
    }
}

impl MongoDbRepo<Chat> {
    async fn watch_messages<F: Into<Document>>(
        &self,
        filter: F,
    ) -> RepoResult<BoxStream<'static, RepoResult<Message>>> {
        let opts = ChangeStreamOptions::builder()
            .full_document(Some(FullDocumentType::Required))
            .build();

        Ok(futures::StreamExt::boxed(futures::StreamExt::filter_map(
            self.database
                .collection(Message::name())
                .watch(vec![filter.into()], opts)
                .await
                .map_err(map_mongo_error)?,
            |e| async move {
                match e {
                    | Ok(event) => match event.full_document {
                        | Some(doc) => Some(Ok(doc)),
                        | None => None,
                    },
                    | Err(err) => Some(Err(map_mongo_error(err))),
                }
            },
        )))
    }
}

impl CollectionEntity for Chat {
    fn name() -> &'static str {
        "chats"
    }
}
