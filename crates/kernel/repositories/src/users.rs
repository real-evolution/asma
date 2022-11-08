use kernel_entities::entities::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait UsersRepo: Interface {
    async fn get_by_id(&self, id: &UserKey) -> RepoResult<User>;
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;
    async fn get_all_by_level(&self, level: UserLevel) -> RepoResult<Vec<User>>;
}
