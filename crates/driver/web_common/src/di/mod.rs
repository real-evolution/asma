use kernel_services::config::ConfigService;

mod root;
mod services;

use std::sync::Arc;

pub use shaku::*;

pub use root::RootModule as DI;
pub trait ServicesModule: HasComponent<dyn ConfigService> {}

pub fn build_di() -> anyhow::Result<Arc<DI>> {
    let services = services::services_module()?;

    Ok(Arc::new(DI::builder(services).build()))
}
