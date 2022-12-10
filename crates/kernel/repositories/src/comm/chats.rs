use kernel_entities::entities::comm::Chat;

use crate::traits::*;

#[async_trait::async_trait]
pub trait ChannelsRepo: Repo<Entity = Chat> + Send + Sync {}
