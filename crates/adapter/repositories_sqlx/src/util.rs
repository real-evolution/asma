use kernel_repositories::error::RepoError;

pub fn map_sqlx_error(err: sqlx::Error) -> anyhow::Error {
    let mkerr = |err| anyhow::Error::new(err);

    match err {
        sqlx::Error::Io(err) => mkerr(RepoError::Io(err)),
        sqlx::Error::RowNotFound => mkerr(RepoError::NotFound),
        _ => mkerr(RepoError::Data(err.into())),
    }
}
