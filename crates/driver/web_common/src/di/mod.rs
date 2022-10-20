mod root;
mod services;

use kernel_services::config::ConfigService;

use root::*;
use services::*;

use std::sync::Arc;

use shaku::HasComponent;

pub trait ServicesModule: HasComponent<dyn ConfigService> {}

pub fn build_di() -> anyhow::Result<Arc<RootModule>> {
    let services = services_module()?;

    Ok(Arc::new(RootModule::builder(services).build()))
}
