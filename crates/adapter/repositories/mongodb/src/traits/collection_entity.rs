use kernel_entities::traits::Entity;
use kernel_repositories::error::RepoResult;
use mongodb::Collection;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait::async_trait]
pub trait CollectionEntity:
    Entity + Serialize + DeserializeOwned + Unpin
{
    fn name() -> &'static str;

    async fn initialize_collection(
        _collection: &Collection<Self>,
    ) -> RepoResult<()> {
        Ok(())
    }
}
