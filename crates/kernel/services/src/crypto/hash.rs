use shaku::Interface;

use crate::error::AppResult;

#[async_trait::async_trait()]
pub trait CryptoHashService: Interface {
    async fn hash(&self, input: &[u8]) -> AppResult<Vec<u8>>;
    async fn hash_str(&self, input: &str) -> AppResult<String>;

    async fn hash_with_salt(
        &self,
        input: &[u8],
        salt: &[u8],
    ) -> AppResult<Vec<u8>>;

    async fn hash_str_with_salt(
        &self,
        input: &str,
        salt: &str,
    ) -> AppResult<String>;
}
