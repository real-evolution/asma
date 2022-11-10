use std::sync::Arc;

use kernel_entities::entities::auth::*;
use kernel_repositories::{
    AccountsRepo, InsertAccount, InsertRole, InsertUser, RolesRepo,
    TransactionManager, UsersRepo,
};
use kernel_services::{
    auth::access::{AppAccess, AppResource},
    crypto::hash::CryptoHashService,
    error::AppResult,
    setup::{error::SetupError, SetupService},
};
use shaku::Component;

const SYSTEM_USER_USERNAME: &str = "system";
const SYSTEM_USER_DISPLAY_NAME: &str = "System User";
const ROOT_ACCOUNT_NAME: &str = "root";

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
    async fn create_system_user(&self) -> AppResult<UserKey> {
        let system_user_id = self
            .users
            .create(InsertUser::new_active(
                SYSTEM_USER_USERNAME.to_owned(),
                SYSTEM_USER_DISPLAY_NAME.to_owned(),
                UserLevel::Root,
            ))
            .await?;

        Ok(system_user_id)
    }

    async fn create_root_account(
        &self,
        user_id: &UserKey,
        root_holder_name: Option<String>,
        root_password: String,
    ) -> AppResult<AccountKey> {
        let root_account_id = self
            .accounts
            .create_for(
                user_id,
                InsertAccount::new_active(
                    ROOT_ACCOUNT_NAME.to_owned(),
                    root_holder_name,
                    self.hash_svc.hash(&root_password)?,
                ),
            )
            .await?;

        Ok(root_account_id)
    }

    async fn setup_roles(&self, root_account_id: &AccountKey) -> AppResult<()> {
        let all_roles = AppResource::get_all()
            .iter()
            .flat_map(|r| AppAccess::new_full(r.clone()).into_string_map());

        for (code, friendly_name) in all_roles {
            let role_id = self
                .roles
                .create(InsertRole {
                    code,
                    friendly_name: Some(friendly_name),
                })
                .await?;

            self.roles.add_to_role(root_account_id, &role_id).await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait()]
impl SetupService for AppSetupService {
    async fn is_setup(&self) -> AppResult<bool> {
        Ok(!self
            .users
            .get_all_by_level(UserLevel::Root)
            .await?
            .is_empty())
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
