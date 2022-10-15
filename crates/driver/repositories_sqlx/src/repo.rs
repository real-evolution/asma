use std::ops::{Deref, DerefMut};

use kernel_entities::traits::BasicEntity;
use kernel_repositories::Repo;

pub type DbType = sqlx::postgres::Postgres;

pub struct SqlxRepo {
    pool: sqlx::Pool<DbType>,
}

impl<E, K> Repo<E, K> for SqlxRepo where E: BasicEntity<Key = K> {}

impl Deref for SqlxRepo {
    type Target = sqlx::Pool<DbType>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for SqlxRepo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}
