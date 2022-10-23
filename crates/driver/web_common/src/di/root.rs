use super::{AdapterServicesModule, AppServicesModule};
use kernel_services::{auth::AuthService, config::ConfigService};

use shaku::module;

module! {
    pub RootModule {
        components = [],
        providers = [],

        use dyn AdapterServicesModule {
            components = [dyn ConfigService],
            providers = [],
        },

        use dyn AppServicesModule {
            components = [dyn AuthService],
            providers = [],
        }
    }
}
