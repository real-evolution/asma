use chrono::{DateTime, Utc};
use kernel_entities::traits::{Entity, Key};

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait Repo<E: Entity> {
    async fn get(&self, key: &Key<E>) -> RepoResult<E>;
    async fn get_paginated(
        &self,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<E>>;

    async fn remove(&self, key: &Key<E>) -> RepoResult<()>;
}

#[async_trait::async_trait]
pub trait InsertRepo<E: Entity, I> {
    async fn create(&self, model: I) -> RepoResult<E>;
}

#[async_trait::async_trait]
pub trait ChildRepo<E: Entity, P: Entity> {
    async fn get_paginated_for(
        &self,
        parent_key: &Key<P>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<E>>;

    async fn get_for(&self, key: &Key<E>, parent_key: &Key<P>)
        -> RepoResult<E>;

    async fn remove_for(
        &self,
        key: &Key<E>,
        parent_key: &Key<P>,
    ) -> RepoResult<()>;
}
