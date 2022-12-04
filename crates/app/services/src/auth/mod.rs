pub mod config;

use std::sync::Arc;

use chrono::{Duration, Utc};
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
use kernel_repositories::auth::*;
use kernel_repositories::error::RepoError;
use kernel_services::auth::models::AccessRule;
use kernel_services::auth::{models::DeviceInfo, AuthService};
use kernel_services::crypto::hash::CryptoHashService;
use kernel_services::entropy::EntropyService;
use kernel_services::error::{AppResult, AuthError};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = AuthService)]
pub struct AppAuthService {
    config: config::AuthConfig,

    #[shaku(inject)]
    users: Arc<dyn UsersRepo>,

    #[shaku(inject)]
    accounts: Arc<dyn AccountsRepo>,

    #[shaku(inject)]
    roles: Arc<dyn RolesRepo>,

    #[shaku(inject)]
    sessions: Arc<dyn SessionsRepo>,

    #[shaku(inject)]
    hash_svc: Arc<dyn CryptoHashService>,

    #[shaku(inject)]
    entropy_svc: Arc<dyn EntropyService>,
}

#[async_trait::async_trait]
impl AuthService for AppAuthService {
    async fn signin(
        &self,
        account_name: &str,
        username: &str,
        password: &str,
        device_info: DeviceInfo,
    ) -> AppResult<(User, Account, Session)> {
        let user = self.users.get_by_username(username).await?;

        if !user.is_active {
            return Err(AuthError::InactiveUser {
                username: username.into(),
                account_name: account_name.into(),
            }
            .into());
        }

        let account = self
            .accounts
            .get_of_user_by_name(&user.id, account_name)
            .await?;

        let AccountState::Active = account.state else {
            return Err(AuthError::InactiveAccount {
                username: username.into(),
                account_name: account_name.into(),
            }
            .into());
        };

        if let Err(err) = self.hash_svc.verify(password, &account.password_hash)
        {
            warn!("could not verify password of `{account_name}@{username}`: {err}");
            return Err(AuthError::InvalidCredentials.into());
        }

        if let Ok(session) = self
            .sessions
            .get_active_for(&account.id, &device_info.device_identifier)
            .await
        {
            self.sessions
                .update(
                    &session.id,
                    &device_info.last_address,
                    &device_info.agent,
                    Duration::seconds(self.config.refresh_validity_seconds),
                )
                .await?;

            info!(
                "`{}@{}` signed-in with a saved session `{:#?}`",
                account_name, username, session.id
            );

            return Ok((user, account, session));
        }

        if self.sessions.get_active_count_for(&account.id).await?
            >= self.config.max_sessions_count
        {
            warn!(
                "`{}@{}` has reached maximum sessions acount of {}",
                account_name, username, self.config.max_sessions_count
            );

            return Err(AuthError::MaxSessionsCountReached(
                self.config.max_sessions_count,
            )
            .into());
        }

        let session = self
            .sessions
            .create(InsertSession {
                account_id: account.id.clone(),
                device_identifier: device_info.device_identifier,
                agent: device_info.agent,
                address: device_info.last_address,
                expires_at: Some(
                    Utc::now()
                        + Duration::seconds(
                            self.config.signin_validity_seconds,
                        ),
                ),
                refresh_token: self
                    .entropy_svc
                    .next_string(self.config.refresh_token_length)?,
            })
            .await?;

        Ok((user, account, session))
    }

    async fn refresh_session(
        &self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<Session> {
        let session = self
            .get_session_by_token(refresh_token, &device_info.device_identifier)
            .await?;

        self.sessions
            .update(
                &session.id,
                &device_info.last_address,
                &device_info.agent,
                Duration::seconds(self.config.refresh_validity_seconds),
            )
            .await?;

        Ok(session)
    }

    async fn invalidate_session(
        &self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<()> {
        let session = self
            .get_session_by_token(refresh_token, device_identifier)
            .await?;

        Ok(self.sessions.remove(&session.id).await?)
    }

    async fn get_access_rules_for(
        &self,
        account_id: &Key<Account>,
    ) -> AppResult<Vec<AccessRule>> {
        Ok(self
            .roles
            .get_roles_with_permissions_for(account_id)
            .await?
            .into_iter()
            .map(|i| AccessRule::new(i.0, i.1))
            .collect())
    }

    async fn update_password_for(
        &self,
        user_id: &Key<User>,
        account_id: &Key<Account>,
        old_password: &str,
        new_password: &str,
    ) -> AppResult<()> {
        let account = self.accounts.get_of(user_id, account_id).await?;

        if self.hash_svc.hash(old_password)? != account.password_hash {
            return Err(AuthError::OldPasswordWrong.into());
        }

        Ok(self
            .accounts
            .set_password_hash(account_id, self.hash_svc.hash(new_password)?)
            .await?)
    }
}

impl AppAuthService {
    async fn get_session_by_token(
        &self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<Session> {
        match self
            .sessions
            .get_active_by_token(refresh_token, device_identifier)
            .await
        {
            Ok(session) => Ok(session),
            Err(RepoError::NotFound) => Err(AuthError::NotAuthenticated.into()),
            Err(err) => return Err(err.into()),
        }
    }
}
