use derive_more::Constructor;
use kernel_entities::entities::auth::*;
use shaku::Interface;

use crate::{
    error::RepoResult,
    traits::repo::{InsertRepo, Repo},
};

#[async_trait::async_trait]
pub trait UsersRepo:
    Repo<User> + InsertRepo<User, InsertUser> + Interface
{
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;
}

#[derive(Constructor, Debug)]
pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub is_active: bool,
}
