use std::sync::Arc;

use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{auth::*, error::RepoError, TransactionManager};
use kernel_services::{
    crypto::hash::CryptoHashService,
    error::AppResult,
    setup::{error::SetupError, SetupService},
};
use shaku::Component;

const SYSTEM_USER_USERNAME: &str = "system";
const SYSTEM_USER_DISPLAY_NAME: &str = "System User";
const ROOT_ACCOUNT_NAME: &str = "root";
const ROOT_ROLE_DESCRIPTION: &str = "Full system access";

#[derive(Component)]
#[shaku(interface = SetupService)]
pub struct AppSetupService {
    #[shaku(inject)]
    tx_mgr: Arc<dyn TransactionManager>,

    #[shaku(inject)]
    users: Arc<dyn UsersRepo>,

    #[shaku(inject)]
    accounts: Arc<dyn AccountsRepo>,

    #[shaku(inject)]
    roles: Arc<dyn RolesRepo>,

    #[shaku(inject)]
    hash_svc: Arc<dyn CryptoHashService>,
}

impl AppSetupService {
    async fn create_system_user(&self) -> AppResult<Key<User>> {
        let system_user_id = self
            .users
            .create(InsertUser::new(
                SYSTEM_USER_USERNAME.to_owned(),
                SYSTEM_USER_DISPLAY_NAME.to_owned(),
                true,
            ))
            .await?;

        Ok(system_user_id)
    }

    async fn create_root_account(
        &self,
        user_id: &Key<User>,
        root_holder_name: Option<String>,
        root_password: String,
    ) -> AppResult<Key<Account>> {
        let root_account_id = self
            .accounts
            .create_for(
                user_id,
                InsertAccount::new(
                    ROOT_ACCOUNT_NAME.to_owned(),
                    root_holder_name,
                    self.hash_svc.hash(&root_password)?,
                    AccountState::Active,
                ),
            )
            .await?;

        Ok(root_account_id)
    }

    async fn setup_roles(&self, root_account_id: &Key<Account>) -> AppResult<()> {
        // create root role
        let role_id = self
            .roles
            .create(InsertRole::new(
                KnownRoles::Root.to_string(),
                Some(ROOT_ROLE_DESCRIPTION.to_owned()),
            ))
            .await?;

        // set root role permissions
        for res in Resource::all() {
            self.roles
                .add_permission(&role_id, res, Actions::all())
                .await?;
        }

        // add account to root role
        self.roles.add_to(root_account_id, &role_id).await?;

        Ok(())
    }
}

#[async_trait::async_trait()]
impl SetupService for AppSetupService {
    async fn is_setup(&self) -> AppResult<bool> {
        match self.users.get_by_username(SYSTEM_USER_USERNAME).await {
            Ok(_) => Ok(true),
            Err(RepoError::NotFound) => return Ok(false),
            Err(err) => return Err(err.into()),
        }
    }

    async fn setup(
        &self,
        root_holder_name: Option<String>,
        root_password: String,
    ) -> AppResult<()> {
        if self.is_setup().await? {
            return Err(SetupError::AlreadySetup.into());
        }

        let tx = self.tx_mgr.begin().await?;

        {
            let system_user_id = self.create_system_user().await?;
            let root_account_id = self
                .create_root_account(
                    &system_user_id,
                    root_holder_name,
                    root_password,
                )
                .await?;
            self.setup_roles(&root_account_id).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
