use kernel_services::{auth::AuthService, config::ConfigService};

mod adapter_services;
mod app_services;
mod root;

use std::sync::Arc;

pub use shaku::*;

pub use root::RootModule as DI;

pub trait AdapterServicesModule: HasComponent<dyn ConfigService> {}
pub trait AppServicesModule: HasComponent<dyn AuthService> {}

pub fn build_di() -> anyhow::Result<Arc<DI>> {
    let adapter_services = adapter_services::adapter_services_module()?;
    let app_services = app_services::app_services_module()?;

    Ok(Arc::new(
        DI::builder(adapter_services, app_services).build(),
    ))
}
