use kernel_entities::traits::{Entity, Key};

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait Repo<E: Entity> {
    async fn get(&self, key: &Key<E>) -> RepoResult<E>;
    async fn get_all(&self) -> RepoResult<Vec<E>>;
    async fn get_paginated(&self, params: (i64, i64)) -> RepoResult<Vec<E>>;
}
