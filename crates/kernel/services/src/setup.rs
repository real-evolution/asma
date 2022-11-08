use shaku::Interface;

use crate::error::AppResult;

#[async_trait::async_trait()]
pub trait SetupService: Interface {
    async fn is_setup(&self) -> AppResult<bool>;
    async fn setup(&self) -> AppResult<()>;
}
