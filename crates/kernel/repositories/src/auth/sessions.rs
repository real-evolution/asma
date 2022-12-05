use chrono::{DateTime, Duration, Utc};
use kernel_entities::{entities::auth::*, traits::Key};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait SessionsRepo:
    Repo<Session> + InsertRepo<Session, InsertSession> + Send + Sync
{
    async fn get_all_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<Vec<Session>>;

    async fn get_active_for(
        &self,
        account_id: &Key<Account>,
        device_identifier: &str,
    ) -> RepoResult<Session>;

    async fn get_active_count_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<usize>;

    async fn get_active_by_token(
        &self,
        token: &str,
        unique_identifier: &str,
    ) -> RepoResult<Session>;

    async fn update(
        &self,
        id: &Key<Session>,
        address: &str,
        agent: &str,
        validity: Duration,
    ) -> RepoResult<()>;
}

#[derive(Debug)]
pub struct InsertSession {
    pub account_id: Key<Account>,
    pub device_identifier: String,
    pub agent: String,
    pub address: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub refresh_token: String,
}
