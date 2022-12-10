use chrono::{DateTime, Utc};
use kernel_entities::traits::{Entity, Key};

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait Repo {
    type Entity;

    async fn get(&self, key: &Key<Self::Entity>) -> RepoResult<Self::Entity>;
    async fn get_paginated(
        &self,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>>;

    async fn exists(&self, key: &Key<Self::Entity>) -> RepoResult<bool>;
    async fn remove(&self, key: &Key<Self::Entity>) -> RepoResult<()>;
}

#[async_trait::async_trait]
pub trait InsertRepo<I>: Repo {
    async fn create(&self, model: I) -> RepoResult<Self::Entity>;
}

#[async_trait::async_trait]
pub trait ChildRepo<P: Entity>: Repo {
    async fn get_paginated_of(
        &self,
        parent_key: &Key<P>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>>;

    async fn get_of(
        &self,
        parent_key: &Key<P>,
        key: &Key<Self::Entity>,
    ) -> RepoResult<Self::Entity>;

    async fn remove_of(
        &self,
        parent_key: &Key<P>,
        key: &Key<Self::Entity>,
    ) -> RepoResult<()>;
}
