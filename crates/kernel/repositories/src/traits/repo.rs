use kernel_entities::traits::BasicEntity;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait Repo<E: BasicEntity> {
    async fn get(&self, key: &E::Key) -> RepoResult<E>;
}
