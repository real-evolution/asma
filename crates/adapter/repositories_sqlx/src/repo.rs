use async_trait::async_trait;
use kernel_entities::traits::{Entity, Key, KeyType};
use kernel_repositories::{error::RepoResult, traits::repo::Repo};
use ormx::Table;

use crate::{database::PoolType, util::error::map_sqlx_error};

pub trait SqlxRepo<M>: Send + Sync {
    fn pool(&self) -> &PoolType;
}

#[async_trait]
impl<E, M> Repo<E> for dyn SqlxRepo<M>
where
    E: Entity + Send + Sync,
    M: Table<Id = KeyType> + Into<E>,
{
    async fn get(&self, key: &Key<E>) -> RepoResult<E> {
        let item = M::get(self.pool(), key.value())
            .await
            .map_err(map_sqlx_error)?;

        Ok(item.into())
    }

    async fn get_all(&self) -> RepoResult<Vec<E>> {
        todo!()
    }

    async fn get_paginated(&self, params: (i64, i64)) -> RepoResult<Vec<E>> {
        todo!()
    }
}
