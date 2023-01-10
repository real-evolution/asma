use chrono::Utc;
use kernel_entities::entities::comm::Chat;
use kernel_repositories::{
    comm::{ChatsRepo, InsertChat},
    error::RepoResult,
    traits::InsertRepo,
};

use crate::{
    repo::MongoDbRepo, traits::collection_entity::CollectionEntity,
    util::error::map_mongo_error,
};

#[async_trait::async_trait]
impl ChatsRepo for MongoDbRepo<Chat> {}

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

impl CollectionEntity for Chat {
    fn name() -> &'static str {
        "chats"
    }
}
