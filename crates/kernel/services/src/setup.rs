use shaku::Interface;

use crate::error::AppResult;

#[async_trait::async_trait()]
pub trait SetupService: Interface {
    async fn is_setup(&self) -> AppResult<bool>;
    async fn setup(&self, root: models::RootDetails) -> AppResult<()>;
}

pub mod models {
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize)]
    pub struct RootDetails {
        pub display_name: String,
        pub holder_name: Option<String>,
        pub password: String,
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum SetupError {
        #[error("system has already been setup")]
        AlreadySetup,
    }
}
