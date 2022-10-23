use kernel_services::config::ConfigService;

mod adapter_services;
mod root;

use std::sync::Arc;

pub use shaku::*;

pub use root::RootModule as DI;

pub trait AdapterServicesModule: HasComponent<dyn ConfigService> {}

pub fn build_di() -> anyhow::Result<Arc<DI>> {
    let adapter_services = adapter_services::adapter_services_module()?;

    Ok(Arc::new(DI::builder(adapter_services).build()))
}
