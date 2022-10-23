use super::ReposModule;

use adapter_repositories_sqlx::*;

use shaku::module;
use std::sync::Arc;

module! {
    pub(crate) ReposModuleImpl: ReposModule {
        components = [
            SqlxDatabase,
            SqlxUsersRepo,
            SqlxAccountsRepo,
            SqlxRolesRepo,
            SqlxSessionsRepo
        ],
        providers = []
    }
}

pub fn repos_module() -> anyhow::Result<Arc<dyn ReposModule>> {
    Ok(Arc::new(ReposModuleImpl::builder().build()))
}
