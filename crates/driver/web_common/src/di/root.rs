use super::AdapterServicesModule;
use kernel_services::config::ConfigService;

use shaku::module;

module! {
    pub RootModule {
        components = [],
        providers = [],

        use dyn AdapterServicesModule {
            components = [dyn ConfigService],
            providers = [],
        }
    }
}
