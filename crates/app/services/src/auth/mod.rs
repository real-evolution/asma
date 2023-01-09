pub mod config;

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use derive_new::new;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{auth::*, error::RepoError, DataStore};
use kernel_services::{
    auth::{models::DeviceInfo, AuthService},
    config::ConfigService,
    crypto::hash::CryptoHashService,
    entropy::EntropyService,
    error::{AppResult, AuthError},
    Service,
};
use tokio::sync::RwLock;

use self::config::{AuthConfig, AUTH_CONFIG_SECTION};

#[derive(new)]
pub struct AppAuthService<C: ConfigService> {
    data: Arc<dyn DataStore>,
    #[new(default)]
    config: RwLock<config::AuthConfig>,
    config_svc: Arc<C>,
    hash_svc: Arc<dyn CryptoHashService>,
    entropy_svc: Arc<dyn EntropyService>,
}

#[async_trait]
impl<C: ConfigService> AuthService for AppAuthService<C> {
    async fn signin(
        &self,
        account_name: &str,
        username: &str,
        password: &str,
        device_info: DeviceInfo,
    ) -> AppResult<(User, Account, Session)> {
        let user = self.data.auth().users().get_by_username(username).await?;

        if !user.is_active {
            return Err(AuthError::InactiveUser {
                username: username.into(),
                account_name: account_name.into(),
            }
            .into());
        }

        let account = self
            .data
            .auth()
            .accounts()
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
            warn!(
                "could not verify password of `{account_name}@{username}`: \
                 {err}"
            );
            return Err(AuthError::InvalidCredentials.into());
        }

        let AuthConfig {
            max_sessions_count,
            refresh_token_length,
            refresh_validity_seconds,
            signin_validity_seconds,
        } = self.config.read().await.clone();

        if let Ok(session) = self
            .data
            .auth()
            .sessions()
            .get_active_for(&account.id, &device_info.device_identifier)
            .await
        {
            self.data
                .auth()
                .sessions()
                .update(
                    &session.id,
                    &device_info.last_address,
                    &device_info.agent,
                    Duration::seconds(refresh_validity_seconds),
                )
                .await?;

            info!(
                "`{}@{}` signed-in with a saved session `{:#?}`",
                account_name, username, session.id
            );

            return Ok((user, account, session));
        }

        if self
            .data
            .auth()
            .sessions()
            .get_active_count_for(&account.id)
            .await?
            >= max_sessions_count
        {
            warn!(
                "`{}@{}` has reached maximum sessions acount of {}",
                account_name, username, max_sessions_count
            );

            return Err(
                AuthError::MaxSessionsCountReached(max_sessions_count).into()
            );
        }

        let session = self
            .data
            .auth()
            .sessions()
            .create(InsertSession {
                account_id: account.id.clone(),
                device_identifier: device_info.device_identifier,
                agent: device_info.agent,
                address: device_info.last_address,
                expires_at: Some(
                    Utc::now() + Duration::seconds(signin_validity_seconds),
                ),
                refresh_token: self
                    .entropy_svc
                    .next_string(refresh_token_length)?,
            })
            .await?;

        Ok((user, account, session))
    }

    async fn refresh_session(
        &self,
        refresh_token: &str,
        device_info: DeviceInfo,
    ) -> AppResult<Session> {
        let refresh_validity_seconds =
            self.config.read().await.refresh_validity_seconds;

        let session = self
            .get_session_by_token(refresh_token, &device_info.device_identifier)
            .await?;

        self.data
            .auth()
            .sessions()
            .update(
                &session.id,
                &device_info.last_address,
                &device_info.agent,
                Duration::seconds(refresh_validity_seconds),
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

        Ok(self.data.auth().sessions().remove(&session.id).await?)
    }

    async fn add_account_for(
        &self,
        user_id: Key<User>,
        account_name: String,
        holder_name: Option<String>,
        password: String,
        is_active: bool,
    ) -> AppResult<Account> {
        if self
            .data
            .auth()
            .accounts()
            .exists_with_name_for(&user_id, &account_name)
            .await?
        {
            return Err(RepoError::AlreadyExists.into());
        }

        Ok(self
            .data
            .auth()
            .accounts()
            .create(InsertAccount::new(
                user_id,
                account_name.to_owned(),
                holder_name,
                self.hash_svc.hash(&password)?,
                is_active.into(),
            ))
            .await?)
    }

    async fn update_password_for(
        &self,
        user_id: &Key<User>,
        account_id: &Key<Account>,
        old_password: &str,
        new_password: &str,
    ) -> AppResult<()> {
        let account = self
            .data
            .auth()
            .accounts()
            .get_of(user_id, account_id)
            .await?;

        if self.hash_svc.hash(old_password)? != account.password_hash {
            return Err(AuthError::OldPasswordWrong.into());
        }

        Ok(self
            .data
            .auth()
            .accounts()
            .set_password_hash(account_id, self.hash_svc.hash(new_password)?)
            .await?)
    }
}

#[async_trait]
impl<C: ConfigService> Service for AppAuthService<C> {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        let conf = self
            .config_svc
            .get_section::<AuthConfig>(AUTH_CONFIG_SECTION)?;

        *self.config.write().await = conf;

        Ok(())
    }
}

impl<C: ConfigService> AppAuthService<C> {
    async fn get_session_by_token(
        &self,
        refresh_token: &str,
        device_identifier: &str,
    ) -> AppResult<Session> {
        match self
            .data
            .auth()
            .sessions()
            .get_active_by_token(refresh_token, device_identifier)
            .await
        {
            | Ok(session) => Ok(session),
            | Err(RepoError::NotFound) => {
                Err(AuthError::NotAuthenticated.into())
            }
            | Err(err) => Err(err.into()),
        }
    }
}
