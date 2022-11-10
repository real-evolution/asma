pub mod access;
pub mod error;
pub mod models;

use kernel_entities::entities::auth::*;
use shaku::Interface;

use self::access::AppAccess;
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

    async fn get_access_items_for(
        &self,
        account_id: &AccountKey,
    ) -> AppResult<Vec<AppAccess>>;
}
