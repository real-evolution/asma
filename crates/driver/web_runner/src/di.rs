use std::sync::Arc;

use adapter_services::config::TomlConfigService;
use kernel_services::config::ConfigService;

use shaku::{module, HasComponent};

pub trait ServicesModule: HasComponent<dyn ConfigService> {}

module! {
    ServicesModuleImpl: ServicesModule {
        components = [TomlConfigService],
        providers = []
    }
}

module! {
    pub RootModule {
        components = [],
        providers = [],

        use dyn ServicesModule {
            components = [dyn ConfigService],
            providers = [],
        }
    }
}

pub fn build_di_module() -> Arc<RootModule> {
    let services = Arc::new(ServicesModuleImpl::builder().build());
    let root = RootModule::builder(services).build();

    Arc::new(root)
}
