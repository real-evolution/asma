use crate::error::AppResult;

pub trait CryptoHashService: Send + Sync {
    fn hash(&self, plain: &str) -> AppResult<String>;
    fn verify(&self, plain: &str, hash: &str) -> AppResult<()>;
}
