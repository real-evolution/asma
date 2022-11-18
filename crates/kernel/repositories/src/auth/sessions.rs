use chrono::{DateTime, Duration, Utc};
use kernel_entities::entities::auth::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait SessionsRepo: Interface {
    async fn get(&self, id: &SessionKey) -> RepoResult<Session>;

    async fn get_all_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Session>>;

    async fn get_active_for(
        &self,
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> RepoResult<Session>;

    async fn get_active_count_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<usize>;

    async fn get_active_by_token(
        &self,
        token: &str,
        unique_identifier: &str,
    ) -> RepoResult<Session>;

    async fn create_for(
        &self,
        account_id: &AccountKey,
        insert: &InsertSession,
    ) -> RepoResult<SessionKey>;

    async fn update(
        &self,
        id: &SessionKey,
        address: &str,
        agent: &str,
        validity: Duration,
    ) -> RepoResult<()>;

    async fn remove(&self, id: &SessionKey) -> RepoResult<()>;
}

#[derive(Debug)]
pub struct InsertSession {
    pub device_identifier: String,
    pub agent: String,
    pub address: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_token: String,
}
