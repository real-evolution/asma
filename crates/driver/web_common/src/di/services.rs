use super::ServicesModule;
use adapter_services::config::TomlConfigService;

use shaku::module;

module! {
    pub(crate) ServicesModuleImpl: ServicesModule {
        components = [TomlConfigService],
        providers = []
    }
}
