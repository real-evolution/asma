use crate::auth::AuthService;
use crate::config::ConfigService;

use shaku::HasComponent;

pub trait ServicesModule:
    HasComponent<dyn ConfigService> + HasComponent<dyn AuthService>
{
}
