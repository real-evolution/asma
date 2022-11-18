use std::sync::Arc;

use chrono::{Duration, Utc};
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::InsertSession;
use kernel_repositories::{auth::SessionsRepo, error::RepoResult};
use shaku::Component;

use crate::{database::SqlxDatabaseConnection, util::map_sqlx_error};

#[derive(Component)]
#[shaku(interface = SessionsRepo)]
pub struct SqlxSessionsRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl SessionsRepo for SqlxSessionsRepo {
    async fn get(&self, id: &SessionKey) -> RepoResult<Session> {
        Ok(
            sqlx::query_as::<_, Session>(
                "SELECT * FROM sessions WHERE id = $1",
            )
            .bind(id)
            .fetch_one(self.db.get())
            .await
            .map_err(map_sqlx_error)?,
        )
    }

    async fn get_all_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Session>> {
        Ok(sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE account_id = $1",
        )
        .bind(account_id)
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn get_active_for(
        &self,
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> RepoResult<Session> {
        Ok(sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE account_id = $1 AND
                  device_identifier = $2 AND
                  expires_at > $3"#,
        )
        .bind(account_id)
        .bind(device_identifier)
        .bind(Utc::now())
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn get_active_count_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<usize> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(id) FROM SESSIONS
            WHERE account_id = $1 AND expires_at > $2
                     "#,
            account_id.0,
            Utc::now(),
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(count.unwrap_or(0) as usize)
    }

    async fn get_active_by_token(
        &self,
        token: &str,
        unique_identifier: &str,
    ) -> RepoResult<Session> {
        Ok(sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE refresh_token = $1 AND
                  device_identifier = $2 AND
                  valid_until > $3"#,
        )
        .bind(token)
        .bind(unique_identifier)
        .bind(Utc::now())
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn update(
        &self,
        id: &SessionKey,
        new_address: &str,
        new_agent: &str,
        validity: Duration,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"
            UPDATE sessions SET
                last_address = $1,
                agent = $2,
                expires_at = $3,
                updated_at = $4
            WHERE id = $5"#,
            new_address,
            new_agent,
            Utc::now() + validity,
            Utc::now(),
            id.0,
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn create_for(
        &self,
        account_id: &AccountKey,
        insert: &InsertSession,
    ) -> RepoResult<SessionKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO sessions (
                device_identifier,
                agent,
                last_address,
                refresh_token,
                account_id,
                expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            insert.device_identifier,
            insert.agent,
            insert.address,
            insert.refresh_token,
            account_id.0,
            insert.expires_at,
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(SessionKey(id))
    }

    async fn remove(&self, id: &SessionKey) -> RepoResult<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", id.0)
            .execute(self.db.get())
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }
}
