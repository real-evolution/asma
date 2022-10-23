use std::sync::Arc;

use kernel_repositories::*;
use kernel_services::auth::{models::DeviceInfo, AuthService};
use kernel_services::error::AppResult;

use shaku::Component;

#[derive(Component)]
#[shaku(interface = AuthService)]
pub struct AppAuthService {
    #[shaku(inject)]
    users: Arc<dyn UsersRepo>,

    #[shaku(inject)]
    accounts: Arc<dyn AccountsRepo>,

    #[shaku(inject)]
    roles: Arc<dyn AccountsRepo>,

    #[shaku(inject)]
    sessions: Arc<dyn AccountsRepo>,
}

#[async_trait::async_trait]
impl AuthService for AppAuthService {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: DeviceInfo,
    ) -> AppResult<()> {
        todo!()
    }

    async fn refresh_session(
        &mut self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<()> {
        todo!()
    }

    async fn invalidate_session(
        &mut self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<()> {
        todo!()
    }
}
