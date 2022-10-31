use shaku::Interface;

use crate::error::AppResult;

pub trait CryptoHashService: Interface {
    fn hash(&self, plain: &str) -> AppResult<String>;
    fn verify(&self, plain: &str, hash: &str) -> AppResult<()>;
}
