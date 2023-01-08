use kernel_entities::entities::comm::Message;
use kernel_repositories::comm::MessagesRepo;

use crate::{repo::MongoDbRepo, traits::collection_entity::CollectionEntity};

impl MessagesRepo for MongoDbRepo<Message> {}

impl CollectionEntity for Message {
    fn name() -> &'static str {
        "messages"
    }
}
