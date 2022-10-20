use std::sync::Arc;

use super::ServicesModule;
use adapter_services::config::TomlConfigService;

use shaku::module;

module! {
    pub(crate) ServicesModuleImpl: ServicesModule {
        components = [TomlConfigService],
        providers = []
    }
}

pub fn services_module() -> anyhow::Result<Arc<dyn ServicesModule>> {
    let loaded_config = Box::new(TomlConfigService::load()?);

    Ok(Arc::new(
        ServicesModuleImpl::builder()
            .with_component_override(loaded_config)
            .build(),
    ))
}
