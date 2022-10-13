use kernel_entities::traits::BasicEntity;

pub trait Repo<Key, Entity>
where
    Entity: BasicEntity<Key = Key>,
{
}
