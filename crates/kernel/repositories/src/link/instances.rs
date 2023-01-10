use derive_more::Constructor;
use kernel_entities::{
    entities::{comm::Chat, link::*},
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait InstancesRepo:
    Repo<Entity = Instance> + InsertRepo<InsertInstance> + Send + Sync
{
    async fn get_members_of(
        &self,
        chat_id: &Key<Chat>,
    ) -> RepoResult<Vec<Instance>>;

    async fn get_by_platform_identifier(
        &self,
        channel_id: &Key<Channel>,
        identifier: i64,
    ) -> RepoResult<Instance>;

    async fn get_all(
        &self,
        channel_id: &Key<Channel>,
    ) -> RepoResult<Vec<Instance>>;
}

#[derive(Constructor)]
pub struct InsertInstance {
    pub platform_identifier: i64,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub phone_number: Option<String>,
    pub chat_id: Key<Chat>,
    pub channel_id: Key<Channel>,
}
