use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{
    entities::{auth::User, comm::Chat, link::*},
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

    async fn get_of_user(
        &self,
        user_id: &Key<User>,
        instance_id: &Key<Instance>,
    ) -> RepoResult<Instance>;

    async fn get_by_user_paginated(
        &self,
        user_id: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Instance>>;

    async fn update(
        &self,
        id: &Key<Instance>,
        model: UpdateInstance,
    ) -> RepoResult<()>;
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

#[derive(Constructor)]
pub struct UpdateInstance {
    pub display_name: Option<String>,
    pub phone_number: Option<String>,
}
