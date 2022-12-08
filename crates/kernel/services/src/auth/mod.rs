pub mod error;
pub mod models;

use kernel_entities::{entities::auth::*, traits::Key};

use crate::error::AppResult;

#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: models::DeviceInfo,
    ) -> AppResult<(User, Account, Session)>;

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

    async fn add_account_for(
        &self,
        user_id: Key<User>,
        account_name: String,
        holder_name: Option<String>,
        password: String,
        is_active: bool,
    ) -> AppResult<Account>;

    async fn update_password_for(
        &self,
        user_id: &Key<User>,
        account_id: &Key<Account>,
        old_password: &str,
        new_password: &str,
    ) -> AppResult<()>;
}
