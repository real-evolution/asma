use std::ops::Deref;

use kernel_entities::entities::*;
use kernel_repositories::{SessionsRepo, error::RepoResult};

use crate::{util::map_sqlx_error, SqlxDatabase};

#[async_trait::async_trait]
impl SessionsRepo for SqlxDatabase {
    async fn get_by_id(&self, id: &SessionKey) -> RepoResult<Session> {
        Ok(
            sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = $1")
                .bind(id)
                .fetch_one(self.deref())
                .await
                .map_err(map_sqlx_error)?,
        )
    }

    async fn get_all_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Session>> {
        Ok(sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE user_id = $1 AND account_id = $2",
        )
        .bind(user_id)
        .bind(account_id)
        .fetch_all(self.deref())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn get_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> RepoResult<Session> {
        Ok(sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE user_id = $1 AND
                  account_id = $2 AND
                  device_identifier = $3"#,
        )
        .bind(user_id)
        .bind(account_id)
        .bind(device_identifier)
        .fetch_one(self.deref())
        .await
        .map_err(map_sqlx_error)?)
    }
}
