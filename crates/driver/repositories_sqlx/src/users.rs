use std::ops::Deref;

use kernel_entities::entities::*;
use kernel_repositories::UsersRepo;

use crate::repo::SqlxRepo;

#[async_trait::async_trait]
impl UsersRepo for SqlxRepo {
    async fn get_by_id(&self, id: &UserKey) -> anyhow::Result<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(self.deref())
                .await?,
        )
    }

    async fn get_by_username(&self, username: &str) -> anyhow::Result<User> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
                .bind(username)
                .fetch_one(self.deref())
                .await?,
        )
    }
}
