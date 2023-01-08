use kernel_entities::entities::comm::Message;

use crate::traits::*;

#[async_trait::async_trait]
pub trait MessagesRepo: Repo<Entity = Message> + Send + Sync {}
