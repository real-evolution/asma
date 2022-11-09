use std::sync::Arc;

use app_services::{auth::{
    config::{AuthConfig, AUTH_CONFIG_SECTION},
    AppAuthService, AppAuthServiceParameters,
}, setup::AppSetupService};
use kernel_repositories::di::ReposModule;
use kernel_repositories::*;
use kernel_services::{
    config::ConfigService, crypto::hash::CryptoHashService, di::ServicesModule,
    entropy::EntropyService, get_config,
};
use shaku::module;

use super::base_services::BaseServicesModule;

module! {
    ServicesModuleImpl: ServicesModule {
        components = [AppAuthService, AppSetupService],
        providers = [],

        use dyn BaseServicesModule {
            components = [
                dyn ConfigService,
                dyn EntropyService,
                dyn CryptoHashService
            ],
            providers = [],
        },

        use dyn ReposModule {
            components = [
                dyn TransactionManager,
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
    let config_svc: &dyn ConfigService = base_services.resolve_ref();
    let auth_conf = get_config!(config_svc, AUTH_CONFIG_SECTION => AuthConfig)?;

    let services = ServicesModuleImpl::builder(base_services, repos)
        .with_component_parameters::<AppAuthService>(AppAuthServiceParameters {
            config: auth_conf,
        })
        .build();

    Ok(Arc::new(services))
}
