use async_trait::async_trait;
use kernel_entities::traits::Entity;
use kernel_repositories::{error::RepoResult, traits::repo::Repo};
use ormx::Table;

use crate::{database::PoolType, util::map_sqlx_error};

pub trait SqlxRepo<M>: Send + Sync {
    fn pool(&self) -> &PoolType;
}

#[async_trait]
impl<E, M> Repo<E> for dyn SqlxRepo<M>
where
    E: Entity + Send + Sync,
    E::Key: Into<E::KeyInner> + Send + Sync,
    E::KeyInner: Send + Sync,
    M: Table<Id = E::KeyInner> + Into<E>,
{
    async fn get(&self, key: &E::Key) -> RepoResult<E> {
        let item = M::get(self.pool(), (*key).into())
            .await
            .map_err(map_sqlx_error)?;

        Ok(item.into())
    }
}
