use derive_more::{Error, Display};

#[derive(Debug, Display, Error)]
pub enum RepoError<K> {
    #[display(fmt = "item with key `{}` not found", _0)]
    NotFound(K),
}
