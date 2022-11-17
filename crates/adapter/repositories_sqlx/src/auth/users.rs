use std::sync::Arc;

use kernel_entities::entities::auth::*;
use kernel_repositories::{auth::{UsersRepo, InsertUser}, error::RepoResult};
use shaku::Component;

use crate::{database::SqlxDatabaseConnection, util::map_sqlx_error};

#[derive(Component)]
#[shaku(interface = UsersRepo)]
pub struct SqlxUsersRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl UsersRepo for SqlxUsersRepo {
    async fn get(&self, id: &UserKey) -> RepoResult<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(self.db.get())
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
            .fetch_one(self.db.get())
            .await
            .map_err(map_sqlx_error)?,
        )
    }

    async fn get_all_by_level(
        &self,
        level: UserLevel,
    ) -> RepoResult<Vec<User>> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE level = $1")
                .bind(level)
                .fetch_all(self.db.get())
                .await
                .map_err(map_sqlx_error)?,
        )
    }

    async fn create(&self, insert: InsertUser) -> RepoResult<UserKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (
                username,
                display_name,
                level,
                is_active)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            insert.username,
            insert.display_name,
            insert.level as i32,
            insert.is_active,
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(UserKey(id))
    }
}
