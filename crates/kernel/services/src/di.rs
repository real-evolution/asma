use shaku::HasComponent;

use crate::auth::AuthService;
use crate::config::ConfigService;
use crate::setup::SetupService;

pub trait ServicesModule:
    HasComponent<dyn ConfigService>
    + HasComponent<dyn SetupService>
    + HasComponent<dyn AuthService>
{
}
