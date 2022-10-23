use crate::error::AppResult;

use shaku::Interface;

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

pub mod models {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct DeviceInfo {
        pub device_identifier: String,
        pub agent: String,
        pub last_address: Option<String>,
        pub last_access: DateTime<Utc>,
    }
}
