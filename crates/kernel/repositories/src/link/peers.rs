use kernel_entities::entities::link::*;

use crate::traits::*;

#[async_trait::async_trait]
pub trait PeersRepo: Repo<Entity = Peer> + Send + Sync {}
