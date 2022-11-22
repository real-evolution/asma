use std::sync::Arc;

use chrono::{DateTime, Utc};
use kernel_entities::entities::auth::*;
use kernel_repositories::{
    auth::{InsertUser, UsersRepo},
    error::RepoResult,
    traits::repo::Repo,
};
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

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<User>> {
        Ok(sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(pagination.0)
        .bind(pagination.1 as i64)
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn create(&self, insert: InsertUser) -> RepoResult<UserKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (
                username,
                display_name,
                is_active)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            insert.username,
            insert.display_name,
            insert.is_active,
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(UserKey(id))
    }

    async fn remove(&self, user_id: &UserKey) -> RepoResult<()> {
        sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, user_id.0)
            .execute(self.db.get())
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Repo<User> for SqlxUsersRepo {
    async fn get(&self, id: &UserKey) -> RepoResult<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(self.db.get())
                .await
                .map_err(map_sqlx_error)?,
        )
    }
}
