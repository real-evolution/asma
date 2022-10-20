mod root;
mod services;

use kernel_services::config::ConfigService;
use root::RootModule;
use services::ServicesModuleImpl;

use std::sync::Arc;

use shaku::HasComponent;

pub trait ServicesModule: HasComponent<dyn ConfigService> {}

pub fn build_di() -> Arc<RootModule> {
    let services = Arc::new(ServicesModuleImpl::builder().build());

    Arc::new(RootModule::builder(services).build())
}
