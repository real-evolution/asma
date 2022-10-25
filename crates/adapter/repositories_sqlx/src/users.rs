use std::sync::Arc;

use kernel_entities::entities::*;
use kernel_repositories::{error::RepoResult, UsersRepo};
use shaku::Component;

use crate::{util::map_sqlx_error, DatabaseConnection};

#[derive(Component)]
#[shaku(interface = UsersRepo)]
pub struct SqlxUsersRepo {
    #[shaku(inject)]
    db: Arc<dyn DatabaseConnection>,
}

#[async_trait::async_trait]
impl UsersRepo for SqlxUsersRepo {
    async fn get_by_id(&self, id: &UserKey) -> RepoResult<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(self.db.deref())
                .await
                .map_err(map_sqlx_error)?,
        )
    }

    async fn get_by_username(&self, username: &str) -> RepoResult<User> {
        Ok(
            sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE username = $1",
            )
            .bind(username)
            .fetch_one(self.db.deref())
            .await
            .map_err(map_sqlx_error)?,
        )
    }
}
