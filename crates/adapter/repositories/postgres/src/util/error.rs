use kernel_repositories::error::RepoError;

pub fn map_sqlx_error(err: sqlx::Error) -> RepoError {
    match err {
        sqlx::Error::Io(err) => RepoError::Io(err),
        sqlx::Error::RowNotFound => RepoError::NotFound,
        _ => RepoError::Data(err.into()),
    }
}
