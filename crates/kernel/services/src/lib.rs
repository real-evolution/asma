use error::AppResult;

pub mod auth;
pub mod config;
pub mod crypto;
pub mod entropy;
pub mod error;
pub mod link;
pub mod setup;

#[async_trait::async_trait]
pub trait Service {
    async fn initialize(&self) -> AppResult<()> {
        Ok(())
    }
}
