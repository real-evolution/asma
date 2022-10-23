use super::AppServicesModule;
use app_services::auth::AppAuthService;

use shaku::module;
use std::sync::Arc;

module! {
    pub(crate) AppServicesModuleImpl: AppServicesModule {
        components = [AppAuthService],
        providers = []
    }
}

pub fn app_services_module() -> anyhow::Result<Arc<dyn AppServicesModule>> {
    Ok(Arc::new(AppServicesModuleImpl::builder().build()))
}
