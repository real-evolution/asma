mod adapter_services;
mod app_services;
mod repos;
mod root;

use adapter_repositories_sqlx::DbConnection;
use kernel_repositories::*;
use kernel_services::{auth::AuthService, config::ConfigService};

use std::sync::Arc;

pub use shaku::*;

pub use root::RootModule as DI;

pub trait ReposModule:
    HasComponent<dyn DbConnection>
    + HasComponent<dyn UsersRepo>
    + HasComponent<dyn AccountsRepo>
    + HasComponent<dyn RolesRepo>
    + HasComponent<dyn SessionsRepo>
{
}

pub trait AdapterServicesModule: HasComponent<dyn ConfigService> {}
pub trait AppServicesModule: HasComponent<dyn AuthService> {}

pub fn build_di() -> anyhow::Result<Arc<DI>> {
    let repos = repos::repos_module()?;

    let adapter_services = adapter_services::adapter_services_module()?;
    let app_services = app_services::app_services_module(repos.clone())?;

    Ok(Arc::new(
        DI::builder(repos, adapter_services, app_services).build(),
    ))
}
