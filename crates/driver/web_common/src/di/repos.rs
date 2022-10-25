use std::sync::Arc;

use adapter_repositories_sqlx::*;
use kernel_repositories::di::ReposModule;
use shaku::{module, HasComponent};

pub trait DatabaseModule: HasComponent<dyn DatabaseConnection> {}

module! {
    pub DatabaseModuleImpl: DatabaseModule {
        components = [ SqlxDatabaseConnection ],
        providers = [],
    }
}

module! {
    pub ReposModuleImpl: ReposModule {
        components = [
            SqlxUsersRepo,
            SqlxAccountsRepo,
            SqlxRolesRepo,
            SqlxSessionsRepo
        ],
        providers = [],

        use dyn DatabaseModule {
            components = [ dyn DatabaseConnection ],
            providers = [],
        },

    }
}

pub async fn database_module(
    conf: DataConfig,
) -> anyhow::Result<Arc<dyn DatabaseModule>> {
    tracing::debug!(
        "openning database connection to: {}",
        conf.get_concealed_connection_string()?
    );

    let module = Arc::new(
        DatabaseModuleImpl::builder()
            .with_component_parameters::<SqlxDatabaseConnection>(
                SqlxDatabaseConnectionParameters {
                    pool: conf.into_pool().await?,
                },
            )
            .build(),
    );

    Ok(module)
}

pub fn repos_module(
    database: Arc<dyn DatabaseModule>,
) -> anyhow::Result<Arc<dyn ReposModule>> {
    Ok(Arc::new(ReposModuleImpl::builder(database).build()))
}
