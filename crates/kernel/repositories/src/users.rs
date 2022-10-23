use crate::error::RepoResult;
use kernel_entities::entities::*;

use shaku::Interface;

#[async_trait::async_trait]
pub trait UsersRepo: Interface {
    async fn get_by_id(&self, id: &UserKey) -> RepoResult<User>;
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;
}
