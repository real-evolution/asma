use kernel_entities::entities::*;

use chrono::{DateTime, Utc};
use derive_more::Display;
use thiserror::Error;

use crate::Service;

#[derive(Debug, Error, Display)]
pub enum AuthError {
    #[display(fmt = "invalid username or password")]
    InvalidUsernameOrPassword,

    #[display(fmt = "maximum number of seassons ({}) has been reached", _0)]
    MaxSessionsCountReached(u32),
}

#[async_trait::async_trait]
pub trait AuthService: Service {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: DeviceInfo,
    ) -> anyhow::Result<Session>;

    async fn refresh_session(
        &mut self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> anyhow::Result<Session>;

    async fn invalidate_session(
        &mut self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> anyhow::Result<Session>;
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub device_identifier: String,
    pub agent: String,
    pub last_address: Option<String>,
    pub last_access: DateTime<Utc>,
}
