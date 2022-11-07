use std::sync::Arc;

use chrono::{Duration, Utc};
use kernel_entities::entities::*;
use kernel_repositories::{error::RepoResult, InsertSession, SessionsRepo};
use shaku::Component;

use crate::{util::map_sqlx_error, DatabaseConnection};

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
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> RepoResult<Session> {
        Ok(sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE account_id = $2 AND
                  device_identifier = $3 AND
                  valid_until > $4"#,
        )
        .bind(account_id)
        .bind(device_identifier)
        .bind(chrono::Utc::now())
        .fetch_one(self.db.deref())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn get_active_sessions_count(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
    ) -> RepoResult<usize> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(id) FROM SESSIONS
            WHERE user_id = $1 AND account_id = $2
                     "#,
            user_id.0,
            account_id.0
        )
        .fetch_one(self.db.deref())
        .await
        .map_err(map_sqlx_error)?
        .count;

        Ok(count.unwrap_or(0) as usize)
    }

    async fn get_optional_valid_by_token(
        &self,
        token: &str,
        unique_identifier: &str,
    ) -> RepoResult<Option<Session>> {
        Ok(sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE refresh_token = $1 AND
                  device_identifier = $2 AND
                  valid_until > $2"#,
        )
        .bind(token)
        .bind(unique_identifier)
        .bind(Utc::now())
        .fetch_optional(self.db.deref())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn update(
        &self,
        id: &SessionKey,
        new_address: &str,
        new_agent: &str,
        validitiy: Duration,
    ) -> RepoResult<()> {
        let now = Utc::now();
        let expires_at = now + validitiy;

        sqlx::query!(
            r#"
            UPDATE sessions SET
                last_access = $1,
                last_address = $2,
                agent = $3,
                updated_at = $4
            WHERE id = $5"#,
            now,
            new_address,
            new_agent,
            expires_at,
            id.0
        )
        .execute(self.db.deref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn create_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
        insert: &InsertSession,
    ) -> RepoResult<SessionKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO sessions (
                device_identifier,
                agent,
                last_address,
                last_access,
                valid_until,
                refresh_token,
                user_id,
                account_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            insert.device_identifier,
            insert.agent,
            insert.address,
            chrono::Utc::now(),
            insert.valid_until,
            insert.refresh_token,
            user_id.0,
            account_id.0
        )
        .fetch_one(self.db.deref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(SessionKey(id))
    }

    async fn remove(&self, id: &SessionKey) -> RepoResult<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", id.0)
            .execute(self.db.deref())
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }
}
