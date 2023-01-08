use chrono::Utc;
use kernel_entities::entities::comm::Message;
use kernel_repositories::{
    comm::{InsertMessage, MessagesRepo},
    error::RepoResult,
    traits::InsertRepo,
};

use crate::{
    repo::MongoDbRepo, traits::collection_entity::CollectionEntity,
    util::error::map_mongo_error,
};

impl MessagesRepo for MongoDbRepo<Message> {}

#[async_trait::async_trait]
impl InsertRepo<InsertMessage> for MongoDbRepo<Message> {
    async fn create(&self, model: InsertMessage) -> RepoResult<Self::Entity> {
        let message = Message {
            id: uuid::Uuid::new_v4().into(),
            text: model.text,
            changes: None,
            attachments: None,
            direction: model.direction,
            delivered_at: None,
            seen_at: None,
            chat_id: model.chat_id,
            created_at: Utc::now(),
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
