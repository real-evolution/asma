use kernel_entities::entities::auth::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait UsersRepo: Interface {
    async fn get_by_id(&self, id: &UserKey) -> RepoResult<User>;
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;
    async fn get_all_by_level(&self, level: UserLevel)
        -> RepoResult<Vec<User>>;

    async fn create(&self, insert: InsertUser) -> RepoResult<UserKey>;
}

#[derive(Debug)]
pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub level: UserLevel,
    pub state: UserState,
}

impl InsertUser {
    pub fn new_active(
        username: String,
        display_name: String,
        level: UserLevel,
    ) -> Self {
        Self {
            username,
            display_name,
            level,
            state: UserState::Active,
        }
    }
}
