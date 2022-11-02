use chrono::{DateTime, Duration, Utc};
use kernel_entities::entities::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait SessionsRepo: Interface {
    async fn get_by_id(&self, id: &SessionKey) -> RepoResult<Session>;

    async fn get_all_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Session>>;

    async fn get_valid_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> RepoResult<Session>;

    async fn get_active_sessions_count(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
    ) -> RepoResult<usize>;

    async fn get_optional_valid_by_token(
        &self,
        token: &str,
        unique_identifier: &str,
    ) -> RepoResult<Option<Session>>;

    async fn update(
        &self,
        id: &SessionKey,
        address: Option<String>,
        agent: &str,
        validitiy: Duration,
    ) -> RepoResult<()>;

    async fn create_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
        insert: &InsertSession,
    ) -> RepoResult<SessionKey>;

    async fn remove(&self, id: &SessionKey) -> RepoResult<()>;
}

#[derive(Debug)]
pub struct InsertSession {
    pub device_identifier: String,
    pub agent: String,
    pub address: Option<String>,
    pub valid_until: DateTime<Utc>,
    pub refresh_token: String,
}
