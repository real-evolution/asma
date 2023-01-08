use std::sync::Arc;

use kernel_entities::entities::comm::Message;
use kernel_repositories::{
    comm::MessagesRepo, error::RepoResult, DocumentStore,
};
use mongodb::{Client, Database};
use repo::MongoDbRepo;
use traits::collection_entity::CollectionEntity;

mod comm;
mod config;
mod repo;
mod traits;
mod util;

pub use config::*;

struct MongoDbDocumentStore {
    _client: Client,
    messages: MongoDbRepo<Message>,
}

impl DocumentStore for MongoDbDocumentStore {
    fn messages(&self) -> &dyn MessagesRepo {
        &self.messages
    }
}

pub async fn create_doc_store(
    conf: config::DocumentStoreConfig,
) -> RepoResult<Arc<dyn DocumentStore>> {
    tracing::debug!(
        "openning a client connection to: {}",
        conf.get_connection_string()?
    );

    let (client, database) = conf.into_client().await?;

    Ok(Arc::new(MongoDbDocumentStore {
        messages: get_initialized_repo(database.clone()).await?,
        _client: client,
    }))
}

async fn get_initialized_repo<E: CollectionEntity>(
    db: Database,
) -> RepoResult<MongoDbRepo<E>> {
    let repo = MongoDbRepo::new(db);

    repo.initialize().await?;

    Ok(repo)
}
