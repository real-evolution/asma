use chrono::Utc;
use kernel_entities::{entities::comm::Message, traits::Key};
use kernel_repositories::{
    comm::{InsertMessage, MessagesRepo},
    error::RepoResult,
    traits::{InsertRepo, Repo},
};
use mongodb::bson::doc;

use crate::{
    repo::MongoDbRepo,
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

impl CollectionEntity for Message {
    fn name() -> &'static str {
        "messages"
    }
}
