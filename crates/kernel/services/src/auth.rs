use crate::error::AppResult;

use chrono::{DateTime, Utc};
use shaku::Interface;

#[async_trait::async_trait]
pub trait AuthService: Interface {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: DeviceInfo,
    ) -> AppResult<()>;

    async fn refresh_session(
        &mut self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<()>;

    async fn invalidate_session(
        &mut self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<()>;
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub device_identifier: String,
    pub agent: String,
    pub last_address: Option<String>,
    pub last_access: DateTime<Utc>,
}
