use kernel_entities::traits::BasicEntity;

use async_trait::async_trait;

#[async_trait]
pub trait Repo<Key, Entity>
where
    Entity: BasicEntity<Key = Key>,
{
    async fn get(id: Key) -> anyhow::Result<Entity>;
}
