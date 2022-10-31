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

    async fn access(
        &self,
        id: &SessionKey,
        address: Option<String>,
        agent: &str,
    ) -> RepoResult<()>;
}
