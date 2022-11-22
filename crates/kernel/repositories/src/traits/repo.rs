use kernel_entities::traits::Entity;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait Repo<E: Entity> {
    async fn get(&self, key: &E::Key) -> RepoResult<E>;
}
