use std::sync::Arc;

use adapter_repositories_sqlx::*;
use kernel_repositories::{di::ReposModule, TransactionManager};
use shaku::{module, HasComponent};

pub trait DatabaseModule:
    HasComponent<dyn SqlxDatabaseConnection> + HasComponent<dyn TransactionManager>
{
}

module! {
    pub DatabaseModuleImpl: DatabaseModule {
        components = [ SqlxPool, SqlxTransactionManager ],
        providers = [],
    }
}

module! {
    pub ReposModuleImpl: ReposModule {
        components = [
            SqlxUsersRepo,
            SqlxAccountsRepo,
            SqlxRolesRepo,
            SqlxSessionsRepo,
            SqlxPool,
        ],
        providers = [],

        use dyn DatabaseModule {
            components = [ dyn TransactionManager ],
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

    let pool = conf.into_pool().await?;
    let module = Arc::new(
        DatabaseModuleImpl::builder()
            .with_component_parameters::<SqlxPool>(SqlxPoolParameters {
                inner: pool.clone(),
            })
            .with_component_parameters::<SqlxTransactionManager>(
                SqlxTransactionManagerParameters { inner: pool },
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
