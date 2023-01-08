use kernel_entities::{
    entities::comm::{Chat, Message},
    traits::Key,
};

use crate::error::AppResult;

#[async_trait::async_trait]
pub trait ChatsService: Send + Sync {
    async fn send_message(
        &self,
        chat_id: &Key<Chat>,
        text: &str,
    ) -> AppResult<()>;
}
