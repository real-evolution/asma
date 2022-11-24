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
