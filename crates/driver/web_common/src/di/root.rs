use super::{AdapterServicesModule, AppServicesModule, ReposModule};

use adapter_repositories_sqlx::DbConnection;
use kernel_repositories::*;
use kernel_services::{auth::AuthService, config::ConfigService};

use shaku::module;

module! {
    pub RootModule {
        components = [],
        providers = [],

        use dyn ReposModule {
            components = [
                dyn DbConnection,
                dyn UsersRepo,
                dyn AccountsRepo,
                dyn RolesRepo,
                dyn SessionsRepo
            ],
            providers = [],
        },

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
