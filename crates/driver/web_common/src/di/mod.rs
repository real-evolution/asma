mod base_services;
mod repos;
mod services;

use std::sync::Arc;

use adapter_repositories_sqlx::config::{DATA_CONST_SECTION, DataConfig};
pub use base_services::base_services_module;
use kernel_services::{config::ConfigService, di::ServicesModule, get_config};

use crate::di::base_services::BaseServicesModule;
pub trait DI = ServicesModule;

pub async fn build_di(
    base_services: Arc<dyn BaseServicesModule>,
) -> anyhow::Result<Arc<dyn DI>> {
    debug!("reading data configuration with key `{DATA_CONST_SECTION}`");
    let config_svc: &dyn ConfigService = base_services.resolve_ref();
    let data_conf = get_config!(config_svc, DATA_CONST_SECTION => DataConfig)?;

    debug!("creating DI parts");
    let data = repos::database_module(data_conf).await?;
    let repos = repos::repos_module(data)?;
    let services = services::build_services(base_services, repos)?;

    Ok(services)
}
