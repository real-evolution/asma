use super::AdapterServicesModule;
use adapter_services::config::TomlConfigService;

use shaku::module;
use std::sync::Arc;

module! {
    pub(crate) AdapterServicesModuleImpl: AdapterServicesModule {
        components = [TomlConfigService],
        providers = []
    }
}

pub fn adapter_services_module(
) -> anyhow::Result<Arc<dyn AdapterServicesModule>> {
    let loaded_config = Box::new(TomlConfigService::load()?);

    Ok(Arc::new(
        AdapterServicesModuleImpl::builder()
            .with_component_override(loaded_config)
            .build(),
    ))
}
