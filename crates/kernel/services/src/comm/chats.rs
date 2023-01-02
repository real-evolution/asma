use kernel_entities::{traits::Key, entities::comm::Chat};

#[async_trait::async_trait]
pub trait ChatsService: Send + Sync {
    async fn append(&self, chat_id: &Key<Chat>);
}
