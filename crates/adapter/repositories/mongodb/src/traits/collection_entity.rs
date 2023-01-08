use kernel_entities::traits::Entity;
use kernel_repositories::error::RepoResult;
use mongodb::Collection;

#[async_trait::async_trait]
pub trait CollectionEntity: Entity + Unpin {
    fn name() -> &'static str;

    async fn initialize_collection(
        _collection: &Collection<Self>,
    ) -> RepoResult<()> {
        Ok(())
    }
}
