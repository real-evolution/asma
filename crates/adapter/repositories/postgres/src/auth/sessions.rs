use proc_macros::Repo;
use chrono::{Duration, Utc};
use derive_more::Constructor;
use kernel_entities::entities::auth::*;
use kernel_entities::traits::*;
use kernel_repositories::auth::*;
use kernel_repositories::error::RepoResult;
use kernel_repositories::traits::*;
use ormx::{Delete, Patch, Table};

use crate::{database::SqlxPool, util::error::map_sqlx_error};

#[derive(Constructor, Repo)]
#[repo(
    table = "sessions",
    read(entity = "Session", model = "models::SessionModel"),
    insert(entity = "InsertSession", model = "models::InsertSessionModel")
)]
pub(crate) struct SqlxSessionsRepo(pub SqlxPool);

#[async_trait::async_trait]
impl SessionsRepo for SqlxSessionsRepo {
    async fn get_all_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<Vec<Session>> {
        Ok(models::SessionModel::by_account_id(
            self.0.get(),
            account_id.value_ref(),
        )
        .await
        .map_err(map_sqlx_error)?
        .into_iter()
        .map(|s| s.into())
        .collect())
    }

    async fn get_active_for(
        &self,
        account_id: &Key<Account>,
        device_identifier: &str,
    ) -> RepoResult<Session> {
        Ok(sqlx::query_as!(
            models::SessionModel,
            r#"
            SELECT * FROM sessions
            WHERE account_id = $1 AND
                  device_identifier = $2 AND
                  expires_at > $3"#,
            account_id.value_ref(),
            device_identifier,
            Utc::now()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?
        .into())
    }

    async fn get_active_count_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<usize> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(id) FROM SESSIONS
            WHERE account_id = $1 AND expires_at > $2"#,
            account_id.value_ref(),
            Utc::now(),
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(count.unwrap_or(0) as usize)
    }

    async fn get_active_by_token(
        &self,
        token: &str,
        unique_identifier: &str,
    ) -> RepoResult<Session> {
        Ok(sqlx::query_as!(
            models::SessionModel,
            r#"
            SELECT * FROM sessions
            WHERE refresh_token = $1 AND
                  device_identifier = $2 AND
                  expires_at > $3"#,
            token,
            unique_identifier,
            Utc::now()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?
        .into())
    }

    async fn update(
        &self,
        id: &Key<Session>,
        new_address: &str,
        new_agent: &str,
        validity: Duration,
    ) -> RepoResult<()> {
        models::UpdateSessionModel {
            agent: new_agent.into(),
            last_address: new_address.into(),
            expires_at: Some(Utc::now() + validity),
            updated_at: Utc::now(),
        }
        .patch_row(self.0.get(), id.value())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::auth::Session, traits::KeyType};
    use kernel_repositories::auth::InsertSession;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "sessions", id = id, insertable, deletable)]
    pub struct SessionModel {
        #[ormx(default)]
        pub id: KeyType,
        #[ormx(get_optional)]
        pub device_identifier: String,
        pub agent: String,
        pub refresh_token: String,
        pub last_address: String,
        #[ormx(get_many)]
        pub account_id: KeyType,
        pub expires_at: Option<DateTime<Utc>>,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    #[derive(ormx::Patch)]
    #[ormx(table_name = "sessions", table = SessionModel, id = "id")]
    pub struct UpdateSessionModel {
        pub last_address: String,
        pub agent: String,
        pub expires_at: Option<DateTime<Utc>>,
        pub updated_at: DateTime<Utc>,
    }

    impl From<InsertSession> for InsertSessionModel {
        fn from(val: InsertSession) -> Self {
            InsertSessionModel {
                account_id: val.account_id.into(),
                device_identifier: val.device_identifier,
                agent: val.agent,
                refresh_token: val.refresh_token,
                last_address: val.address,
                expires_at: val.expires_at,
            }
        }
    }

    generate_mapping!(Session, SessionModel, 9);
}
