use super::ServicesModule;
use kernel_services::config::ConfigService;

use shaku::module;

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
