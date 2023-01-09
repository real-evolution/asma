use std::sync::Arc;

use error::AppResult;

pub mod auth;
pub mod comm;
pub mod config;
pub mod crypto;
pub mod entropy;
pub mod error;
pub mod link;
pub mod setup;

#[async_trait::async_trait]
pub trait Service {
    async fn initialize(self: Arc<Self>) -> AppResult<()>;
}
