use std::sync::Arc;

use adapter_services::{
    config::TomlConfigService, entropy::SecureEntropyService,
};
use kernel_services::{config::ConfigService, entropy::EntropyService};
use shaku::{module, HasComponent};

pub trait BaseServicesModule:
    HasComponent<dyn ConfigService> + HasComponent<dyn EntropyService>
{
}

module! {
    pub BaseServicesModuleImpl: BaseServicesModule {
        components = [ TomlConfigService, SecureEntropyService ],
        providers = [],
    }
}

pub fn base_services_module() -> anyhow::Result<Arc<dyn BaseServicesModule>> {
    let loaded_config = Box::new(TomlConfigService::load()?);

    Ok(Arc::new(
        BaseServicesModuleImpl::builder()
            .with_component_override::<dyn ConfigService>(loaded_config)
            .build(),
    ))
}
