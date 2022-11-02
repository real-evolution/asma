pub mod config;

use std::sync::Arc;

use chrono::{Duration, Utc};
use kernel_repositories::{
    AccountsRepo, InsertSession, SessionsRepo, UsersRepo,
};
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
    ) -> AppResult<()> {
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
                    device_info.last_address.or(session.last_address),
                    &device_info.agent,
                    Duration::seconds(self.config.refresh_validity_seconds),
                )
                .await?;

            info!(
                "`{}@{}` signed-in with a saved session `{:#?}`",
                account_name, username, session.id
            );

            return Ok(());
        }

        if self
            .sessions
            .get_active_sessions_count(&user.id, &account.id)
            .await?
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
            valid_until: Utc::now()
                + Duration::seconds(self.config.signin_validity_seconds),
            refresh_token: self
                .entropy_svc
                .next_string(self.config.refresh_token_length)?,
        };

        self.sessions
            .create_for(&user.id, &account.id, &session)
            .await?;

        Ok(())
    }

    async fn refresh_session(
        &mut self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<()> {
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
                device_info.last_address.or(session.last_address),
                &device_info.agent,
                Duration::seconds(self.config.refresh_validity_seconds),
            )
            .await?;

        Ok(())
    }

    async fn invalidate_session(
        &mut self,
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
}
