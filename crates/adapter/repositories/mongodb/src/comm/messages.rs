use chrono::{DateTime, Utc};
use kernel_entities::{
    entities::comm::{Chat, Message},
    traits::Key,
};
use kernel_repositories::{
    comm::{InsertMessage, MessagesRepo},
    error::{RepoError, RepoResult},
    traits::{ChildRepo, InsertRepo, Repo},
};
use mongodb::{bson::doc, options::FindOptions};
use tokio_stream::StreamExt;

use crate::{
    repo::{MongoDbRepo, ENTITY_CREATED_AT_FIELD, ENTITY_ID_FIELD},
    traits::collection_entity::CollectionEntity,
    util::error::map_mongo_error,
};

#[async_trait::async_trait]
impl MessagesRepo for MongoDbRepo<Message> {
    async fn update_text(
        &self,
        id: &Key<Message>,
        new_text: Option<String>,
    ) -> RepoResult<Message> {
        let message = self.get(id).await?;

        self.collection().update_one(
            doc! {"id": message.id.to_string()},
            doc! {"$set": {
                    "updated_at": Utc::now(),
                    format!("changes.{}.content", message.changes.len()): new_text,
                }
            },
            None,
        ).await.map_err(map_mongo_error)?;

        Ok(message)
    }
}

#[async_trait::async_trait]
impl InsertRepo<InsertMessage> for MongoDbRepo<Message> {
    async fn create(&self, model: InsertMessage) -> RepoResult<Self::Entity> {
        let message = Message {
            id: uuid::Uuid::new_v4().into(),
            text: model.text,
            changes: Vec::new(),
            attachments: Vec::new(),
            direction: model.direction,
            delivered_at: model.delivered_at,
            seen_at: None,
            user_id: model.user_id,
            chat_id: model.chat_id,
            instance_id: model.instance_id,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.collection()
            .insert_one(&message, None)
            .await
            .map_err(map_mongo_error)?;

        Ok(message)
    }
}

#[async_trait::async_trait]
impl ChildRepo<Chat> for MongoDbRepo<Message> {
    async fn get_paginated_of(
        &self,
        parent_key: &Key<Chat>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        self.find_stream(
            doc! {ENTITY_CREATED_AT_FIELD: {"$lt": before}, "chat_id": parent_key.value_ref()},
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
        parent_key: &Key<Chat>,
        key: &Key<Self::Entity>,
    ) -> RepoResult<Self::Entity> {
        self.find_one(doc! {ENTITY_ID_FIELD: key.value_ref(), "chat_id": parent_key.value_ref()}, None)
            .await
    }

    async fn remove_of(
        &self,
        parent_key: &Key<Chat>,
        key: &Key<Self::Entity>,
    ) -> RepoResult<()> {
        let ret = self
            .collection()
            .delete_one(doc! { ENTITY_ID_FIELD: key.value_ref(), "chat_id": parent_key.value_ref()}, None)
            .await
            .map_err(map_mongo_error)?;

        if ret.deleted_count != 1 {
            return Err(RepoError::NotFound);
        }

        Ok(())
    }
}

impl CollectionEntity for Message {
    fn name() -> &'static str {
        "messages"
    }
}
