use derive_more::Constructor;
use kernel_entities::entities::auth::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait UsersRepo: Interface {
    async fn get(&self, id: &UserKey) -> RepoResult<User>;
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;
    async fn create(&self, insert: InsertUser) -> RepoResult<UserKey>;
}

#[derive(Constructor, Debug)]
pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub is_active: bool,
}
