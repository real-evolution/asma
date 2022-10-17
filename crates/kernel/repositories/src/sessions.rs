use kernel_entities::entities::*;

use crate::Repo;

#[async_trait::async_trait]
pub trait SessionsRepo: Repo<Session, SessionKey> {
    async fn get_by_id(&self, id: &SessionKey) -> anyhow::Result<Session>;

    async fn get_all_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
    ) -> anyhow::Result<Vec<Session>>;

    async fn get_for(
        &self,
        user_id: &UserKey,
        account_id: &AccountKey,
        device_identifier: &str,
    ) -> anyhow::Result<Session>;
}
