use kernel_entities::traits::BasicEntity;

pub trait Repo<E, K>
where
    E: BasicEntity<Key = K>,
{
}
