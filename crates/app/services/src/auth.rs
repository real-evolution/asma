use kernel_entities::entities::Session;
use kernel_services::auth::{AuthService, DeviceInfo};
use kernel_services::error::AppResult;

use shaku::Component;

#[derive(Component)]
#[shaku(interface = AuthService)]
pub struct AppAuthService;

#[async_trait::async_trait]
impl AuthService for AppAuthService {
    async fn signin(
        &self,
        account_name: &str,
        usrename: &str,
        password: &str,
        device_info: DeviceInfo,
    ) -> AppResult<Session> {
        todo!()
    }

    async fn refresh_session(
        &mut self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<Session> {
        todo!()
    }

    async fn invalidate_session(
        &mut self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<Session> {
        todo!()
    }
}
