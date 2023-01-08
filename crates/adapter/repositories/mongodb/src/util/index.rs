use kernel_repositories::error::RepoResult;
use mongodb::{bson::Document, options::IndexOptions, Collection, IndexModel};

use crate::traits::collection_entity::CollectionEntity;
use crate::util::error::map_mongo_error;

pub async fn create_index<T: CollectionEntity>(
    collection: &Collection<T>,
    keys: Document,
    options: Option<IndexOptions>,
) -> RepoResult<()> {
    collection
        .create_index(
            IndexModel::builder().keys(keys).options(options).build(),
            None,
        )
        .await
        .map_err(map_mongo_error)?;

    Ok(())
}
