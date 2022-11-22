pub mod error;
pub mod models;

use kernel_entities::{entities::auth::*, traits::Key};
use shaku::Interface;

use self::models::AccessRule;
use crate::error::AppResult;

#[async_trait::async_trait]
pub trait AuthService: Interface {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: models::DeviceInfo,
    ) -> AppResult<Session>;

    async fn refresh_session(
        &self,
        refresh_token: &str,
        device_info: models::DeviceInfo,
    ) -> AppResult<Session>;

    async fn invalidate_session(
        &self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<()>;

    async fn get_access_rules_for(
        &self,
        account_id: &Key<Account>,
    ) -> AppResult<Vec<AccessRule>>;
}
