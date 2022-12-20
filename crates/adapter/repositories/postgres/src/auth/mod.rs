use kernel_repositories::auth::*;

mod accounts;
mod roles;
mod sessions;
mod users;

use crate::database::SqlxPool;

pub(crate) struct SqlxAuthDataStore {
    users: users::SqlxUsersRepo,
    accounts: accounts::SqlxAccountsRepo,
    roles: roles::SqlxRolesRepo,
    sessions: sessions::SqlxSessionsRepo,
}

impl SqlxAuthDataStore {
    pub(crate) fn new(pool: SqlxPool) -> Self {
        Self {
            users: users::SqlxUsersRepo(pool.clone()),
            accounts: accounts::SqlxAccountsRepo(pool.clone()),
            roles: roles::SqlxRolesRepo(pool.clone()),
            sessions: sessions::SqlxSessionsRepo(pool),
        }
    }
}

impl AuthDataStore for SqlxAuthDataStore {
    fn users(&self) -> &dyn UsersRepo {
        &self.users
    }

    fn roles(&self) -> &dyn RolesRepo {
        &self.roles
    }

    fn accounts(&self) -> &dyn AccountsRepo {
        &self.accounts
    }

    fn sessions(&self) -> &dyn SessionsRepo {
        &self.sessions
    }
}
