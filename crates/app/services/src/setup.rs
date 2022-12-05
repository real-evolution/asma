use std::sync::Arc;

use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{auth::*, error::RepoError, DataStore};
use kernel_services::{
    crypto::hash::CryptoHashService,
    error::AppResult,
    setup::{error::SetupError, SetupService},
};

const SYSTEM_USER_USERNAME: &str = "system";
const SYSTEM_USER_DISPLAY_NAME: &str = "System User";
const ROOT_ACCOUNT_NAME: &str = "root";
const ROOT_ROLE_DESCRIPTION: &str = "Full system access";

#[derive(Constructor)]
pub struct AppSetupService {
    data: Arc<dyn DataStore>,
    hash_svc: Arc<dyn CryptoHashService>,
}

impl AppSetupService {
    async fn create_system_user(&self) -> AppResult<User> {
        let system_user = self
            .data
            .auth()
            .users()
            .create(InsertUser::new(
                SYSTEM_USER_USERNAME.to_owned(),
                SYSTEM_USER_DISPLAY_NAME.to_owned(),
                true,
            ))
            .await?;

        Ok(system_user)
    }

    async fn create_root_account(
        &self,
        user_id: Key<User>,
        root_holder_name: Option<String>,
        root_password: String,
    ) -> AppResult<Account> {
        let root_account = self
            .data
            .auth()
            .accounts()
            .create(InsertAccount::new(
                user_id,
                ROOT_ACCOUNT_NAME.to_owned(),
                root_holder_name,
                self.hash_svc.hash(&root_password)?,
                AccountState::Active,
            ))
            .await?;

        Ok(root_account)
    }

    async fn setup_roles(
        &self,
        root_account_id: &Key<Account>,
    ) -> AppResult<()> {
        // create root role
        let role = self
            .data
            .auth()
            .roles()
            .create(InsertRole::new(
                KnownRoles::Root.to_string(),
                Some(ROOT_ROLE_DESCRIPTION.to_owned()),
            ))
            .await?;

        // add account to root role
        self.data
            .auth()
            .roles()
            .add_to(root_account_id, &role.id)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl SetupService for AppSetupService {
    async fn is_setup(&self) -> AppResult<bool> {
        match self
            .data
            .auth()
            .users()
            .get_by_username(SYSTEM_USER_USERNAME)
            .await
        {
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

        let tx = self.data.tx().begin().await?;

        {
            let system_user = self.create_system_user().await?;
            let root_account = self
                .create_root_account(
                    system_user.id,
                    root_holder_name,
                    root_password,
                )
                .await?;
            self.setup_roles(&root_account.id).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
