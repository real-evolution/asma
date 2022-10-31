use crate::{util::map_sqlx_error, DatabaseConnection};

use kernel_entities::entities::*;
use kernel_repositories::{error::RepoResult, SessionsRepo};
use shaku::Component;

use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = SessionsRepo)]
pub struct SqlxSessionsRepo {
    #[shaku(inject)]
    db: Arc<dyn DatabaseConnection>,
}

#[async_trait::async_trait]
impl SessionsRepo for SqlxSessionsRepo {
    async fn get_by_id(&self, id: &SessionKey) -> RepoResult<Session> {
        Ok(
            sqlx::query_as::<_, Session>(
                "SELECT * FROM sessions WHERE id = $1",
            )
            .bind(id)
            .fetch_one(self.db.deref())
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
        .fetch_all(self.db.deref())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn get_valid_for(
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
                  device_identifier = $3 AND
                  valid_until > $4"#,
        )
        .bind(user_id)
        .bind(account_id)
        .bind(device_identifier)
        .bind(chrono::Utc::now())
        .fetch_one(self.db.deref())
        .await
        .map_err(map_sqlx_error)?)
    }
}
