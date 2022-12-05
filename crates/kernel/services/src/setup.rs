use crate::error::AppResult;

#[async_trait::async_trait]
pub trait SetupService: Send + Sync {
    async fn is_setup(&self) -> AppResult<bool>;
    async fn setup(
        &self,
        root_holder_name: Option<String>,
        root_password: String,
    ) -> AppResult<()>;
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum SetupError {
        #[error("system has already been setup")]
        AlreadySetup,
    }
}
