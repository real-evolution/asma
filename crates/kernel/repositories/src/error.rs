use thiserror::Error;

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("i/o error: {0}")]
    Io(std::io::Error),

    #[error("data error: {0}")]
    Data(anyhow::Error),

    #[error("no items were found")]
    NotFound,

    #[error("an item with the same unique values already exists")]
    AlreadyExists,

    #[error("duplicate item: {0}")]
    DuplicateValue(String),

    #[error("invlalid parameter: {0}")]
    InvalidParameter(String),

}
