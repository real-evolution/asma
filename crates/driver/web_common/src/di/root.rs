use std::sync::Arc;

use adapter_repositories_sqlx::config::DataConfig;
use adapter_repositories_sqlx::config::DATA_CONST_SECTION;
use kernel_repositories::auth::*;
use kernel_repositories::di::ReposModule;
use kernel_repositories::link::*;
use kernel_repositories::TransactionManager;
use kernel_services::auth::AuthService;
use kernel_services::config::ConfigService;
use kernel_services::crypto::hash::CryptoHashService;
use kernel_services::di::ServicesModule;
use kernel_services::entropy::EntropyService;
use kernel_services::get_config;
use kernel_services::setup::SetupService;
use shaku::module;
use shaku::HasComponent;

use super::base_services::BaseServicesModule;

pub trait RootModule:
    HasComponent<dyn ConfigService>
    + HasComponent<dyn EntropyService>
    + HasComponent<dyn CryptoHashService>
    + HasComponent<dyn AuthService>
    + HasComponent<dyn SetupService>
    + HasComponent<dyn TransactionManager>
    + HasComponent<dyn UsersRepo>
    + HasComponent<dyn AccountsRepo>
    + HasComponent<dyn RolesRepo>
    + HasComponent<dyn SessionsRepo>
    + HasComponent<dyn ChannelsRepo>
    + HasComponent<dyn ConfigService>
{
}

module! {
     RootModuleImpl: RootModule {
        components = [],
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
                dyn SessionsRepo,
                dyn ChannelsRepo,
            ],
            providers = [],
        },

        use dyn ServicesModule {
            components = [
                dyn AuthService,
                dyn SetupService,
            ],
            providers = [],
        },

    }
}

pub async fn build_root(
    base_services: Arc<dyn BaseServicesModule>,
) -> anyhow::Result<Arc<dyn RootModule>> {
    debug!("reading data configuration with key `{DATA_CONST_SECTION}`");
    let config_svc: &dyn ConfigService = base_services.resolve_ref();
    let data_conf = get_config!(config_svc, DATA_CONST_SECTION => DataConfig)?;

    debug!("creating DI parts");
    let data = super::repos::database_module(data_conf).await?;
    let repos = super::repos::repos_module(data)?;
    let services =
        super::services::build_services(base_services.clone(), repos.clone())?;

    Ok(Arc::new(
        RootModuleImpl::builder(base_services, repos, services).build(),
    ))
}
