use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait UsersRepo:
    Repo<Entity = User> + InsertRepo<InsertUser> + Send + Sync
{
    async fn get_by_username(&self, username: &str) -> RepoResult<User>;

    async fn set_display_name(
        &self,
        id: &Key<User>,
        value: String,
    ) -> RepoResult<()>;
}

#[derive(Constructor, Debug)]
pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub is_active: bool,
}
