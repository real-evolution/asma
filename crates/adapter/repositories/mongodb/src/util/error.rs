use kernel_repositories::error::RepoError;
use mongodb::error::ErrorKind;

pub fn map_mongo_error(err: mongodb::error::Error) -> RepoError {
    match *err.kind {
        | ErrorKind::InvalidArgument { message, .. }
        | ErrorKind::InvalidResponse { message, .. } => {
            RepoError::InvalidParameter(message)
        }
        | ErrorKind::BsonDeserialization(err) => {
            RepoError::Deserialization(err.to_string())
        }
        | ErrorKind::BsonSerialization(err) => {
            RepoError::Serialization(err.to_string())
        }
        | _ => RepoError::Data(anyhow::Error::new(err)),
    }
}
