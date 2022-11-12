pub mod config;

use std::sync::Arc;

use chrono::{Duration, Utc};
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::*;
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
    ) -> AppResult<Session> {
        // TODO:
        // validate user & account validity

        let user = self.users.get_by_username(username).await?;
        let account = self
            .accounts
            .get_of_user_by_name(&user.id, account_name)
            .await?;

        if let Err(err) = self.hash_svc.verify(password, &account.password_hash)
        {
            warn!("could not verify password of `{account_name}@{username}`: {err}");
            return Err(AuthError::InvalidCredentials.into());
        }

        if let Ok(session) = self
            .sessions
            .get_valid_for(&account.id, &device_info.device_identifier)
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

            return Ok(session);
        }

        if self.sessions.get_active_sessions_count(&account.id).await?
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

        let session = InsertSession {
            device_identifier: device_info.device_identifier,
            agent: device_info.agent,
            address: device_info.last_address,
            expires_at: Utc::now()
                + Duration::seconds(self.config.signin_validity_seconds),
            refresh_token: self
                .entropy_svc
                .next_string(self.config.refresh_token_length)?,
        };

        let session_id =
            self.sessions.create_for(&account.id, &session).await?;

        Ok(self.sessions.get_by_id(&session_id).await?)
    }

    async fn refresh_session(
        &self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<Session> {
        let session = self
            .sessions
            .get_optional_valid_by_token(
                refresh_token,
                &device_info.device_identifier,
            )
            .await?
            .ok_or(AuthError::NotAuthenticated)?;

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
            .sessions
            .get_optional_valid_by_token(refresh_token, device_identifier)
            .await?
            .ok_or(AuthError::NotAuthenticated)?;

        Ok(self.sessions.remove(&session.id).await?)
    }

    async fn get_access_rules_for(
        &self,
        account_id: &AccountKey,
    ) -> AppResult<Vec<AccessRule>> {
        Ok(self
            .roles
            .get_roles_with_permissions_for(account_id)
            .await?
            .into_iter()
            .map(|i| AccessRule::new(i.0, i.1))
            .collect())
    }
}
