use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};
use shaku::Interface;

use crate::{error::RepoResult, traits::repo::Repo};

#[async_trait::async_trait]
pub trait UsersRepo: Repo<User> + Interface {
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;
    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<User>>;

    async fn create(&self, insert: InsertUser) -> RepoResult<Key<User>>;
    async fn remove(&self, user_id: &Key<User>) -> RepoResult<()>;
}

#[derive(Constructor, Debug)]
pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub is_active: bool,
}
