pub extern crate shaku;

mod root;
mod services;

use kernel_services::config::ConfigService;
use shaku::HasComponent;
use std::sync::Arc;

pub use root::RootModule;
pub trait ServicesModule: HasComponent<dyn ConfigService> {}

pub fn build_di() -> anyhow::Result<Arc<RootModule>> {
    let services = services::services_module()?;

    Ok(Arc::new(RootModule::builder(services).build()))
}
