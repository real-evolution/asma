use super::{AppServicesModule, ReposModule};
use app_services::auth::AppAuthService;

use kernel_repositories::*;
use shaku::module;
use std::sync::Arc;

module! {
    pub(crate) AppServicesModuleImpl: AppServicesModule {
        components = [AppAuthService],
        providers = [],

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

pub fn app_services_module(
    repos_module: Arc<dyn ReposModule>,
) -> anyhow::Result<Arc<dyn AppServicesModule>> {
    Ok(Arc::new(
        AppServicesModuleImpl::builder(repos_module).build(),
    ))
}
