use std::sync::Arc;

use adapter_services::config::TomlConfigService;
use kernel_services::config::ConfigService;
use shaku::{module, HasComponent};

pub trait BaseServicesModule: HasComponent<dyn ConfigService> {}

module! {
    pub BaseServicesModuleImpl: BaseServicesModule {
        components = [ TomlConfigService ],
        providers = [],
    }
}

pub fn base_services_module() -> anyhow::Result<Arc<dyn BaseServicesModule>> {
    let loaded_config = Box::new(TomlConfigService::load()?);

    Ok(Arc::new(
        BaseServicesModuleImpl::builder()
            .with_component_override(loaded_config)
            .build(),
    ))
}
