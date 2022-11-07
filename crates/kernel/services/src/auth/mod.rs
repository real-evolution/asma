pub mod error;
pub mod models;

use shaku::Interface;

use crate::error::AppResult;

#[async_trait::async_trait]
pub trait AuthService: Interface {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: models::DeviceInfo,
    ) -> AppResult<()>;

    async fn refresh_session(
        &mut self,
        refresh_token: &str,
        device_info: models::DeviceInfo,
    ) -> AppResult<()>;

    async fn invalidate_session(
        &mut self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<()>;
}
