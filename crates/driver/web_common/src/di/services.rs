use std::sync::Arc;

use app_services::auth::AppAuthService;
use kernel_repositories::di::ReposModule;
use kernel_repositories::*;
use kernel_services::{config::ConfigService, di::ServicesModule};
use shaku::module;

use super::base_services::BaseServicesModule;

module! {
    ServicesModuleImpl: ServicesModule {
        components = [AppAuthService],
        providers = [],

        use dyn BaseServicesModule {
            components = [ dyn ConfigService ],
            providers = [],
        },

        use dyn ReposModule {
            components = [
                dyn UsersRepo,
                dyn AccountsRepo,
                dyn RolesRepo,
                dyn SessionsRepo
            ],
            providers = [],
        },
    }
}

pub(super) fn build_services(
    base_services: Arc<dyn BaseServicesModule>,
    repos: Arc<dyn ReposModule>,
) -> anyhow::Result<Arc<dyn ServicesModule>> {
    Ok(Arc::new(
        ServicesModuleImpl::builder(base_services, repos).build(),
    ))
}
