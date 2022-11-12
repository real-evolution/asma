use thiserror::Error;

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("i/o error: {0}")]
    Io(std::io::Error),

    #[error("data error: {0}")]
    Data(anyhow::Error),

    #[error("not items were found")]
    NotFound,

    #[error("duplicate item: {0}")]
    DuplicateValue(String),
}
