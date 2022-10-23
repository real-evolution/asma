use kernel_entities::entities::*;

use crate::{Repo, error::RepoResult};

#[async_trait::async_trait]
pub trait SessionsRepo: Repo<Session, SessionKey> {
    async fn get_by_id(&self, id: &SessionKey) -> RepoResult<Session>;

    async fn get_all_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Session>>;

    async fn get_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> RepoResult<Session>;
}
