use kernel_entities::entities::comm::Chat;
use kernel_repositories::comm::ChatsRepo;

use crate::{repo::MongoDbRepo, traits::collection_entity::CollectionEntity};

impl ChatsRepo for MongoDbRepo<Chat> {}

impl CollectionEntity for Chat {
    fn name() -> &'static str {
        "chats"
    }
}
