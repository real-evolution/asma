use shaku::Interface;

use crate::error::AppResult;

#[async_trait::async_trait()]
pub trait SetupService: Interface {
    async fn is_setup(&self) -> AppResult<bool>;
    async fn setup(&self) -> AppResult<()>;
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum SetupError {
        #[error("system has already been setup")]
        AlreadySetup,
    }
}
