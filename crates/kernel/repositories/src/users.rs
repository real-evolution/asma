use kernel_entities::entities::*;

use crate::Repo;

#[async_trait::async_trait]
pub trait UsersRepo: Repo<User, UserKey> {
    async fn get_by_id(&self, id: &UserKey) -> anyhow::Result<User>;
    async fn get_by_username(&self, username: &str) -> anyhow::Result<User>;
}
