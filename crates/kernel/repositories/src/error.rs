use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum RepoError {
    #[display(fmt = "i/o error: {}", _0)]
    Io(std::io::Error),

    #[display(fmt = "data error: {}", _0)]
    Data(anyhow::Error),

    #[display(fmt = "item not found")]
    NotFound,
}
