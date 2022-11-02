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

pub mod models {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct DeviceInfo {
        pub device_identifier: String,
        pub agent: String,
        pub last_address: Option<String>,
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum AuthError {
        #[error("account has no password set")]
        UnsetPassword,

        #[error("invalid credentials")]
        InvalidCredentials,

        #[error("maximum number of seassons ({0}) has been reached")]
        MaxSessionsCountReached(usize),

        #[error("account not withenticated")]
        NotAuthenticated,
    }
}
