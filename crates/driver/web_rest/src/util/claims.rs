use std::{cmp::min, collections::HashMap};

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::{
    Account, Actions, Resource, Session, User,
};
use kernel_entities::traits::Key;
use kernel_services::auth::models::AccessRule;
use serde::{Deserialize, Serialize};

use crate::config::ApiConfig;
use crate::error::{ApiError, ApiResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Key<Session>,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: String,
    pub user_id: Key<User>,
    pub username: String,
    pub display_name: String,
    pub account_id: Key<Account>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub roles: HashMap<String, Vec<(Resource, Actions)>>,
}

impl Claims {
    pub fn new(
        user: User,
        account: Account,
        session: Session,
        access_rules: Vec<AccessRule>,
        config: &ApiConfig,
    ) -> Claims {
        let iat = Utc::now().timestamp();
        let conf_exp = iat + config.token.timout_seconds;
        let exp = match session.expires_at {
            Some(session_exp) => min(conf_exp, session_exp.timestamp()),
            None => conf_exp,
        };

        Claims {
            sub: session.id,
            iat,
            exp,
            iss: config.token.issuer.clone(),
            aud: config.token.audience.clone(),
            user_id: user.id,
            username: user.username,
            display_name: user.display_name,
            account_id: account.id,
            account_name: account.account_name,
            holder_name: account.holder_name,
            roles: access_rules
                .into_iter()
                .map(|a| (a.role_code, a.permissions))
                .collect(),
        }
    }

    pub fn encode(&self, key: &[u8]) -> ApiResult<String> {
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(key),
        )?;

        Ok(jwt)
    }
}

impl Claims {
    pub fn require_role<'a, R: Into<&'a str>>(&self, role: R) -> ApiResult<()> {
        if self.roles.contains_key(role.into()) {
            return Ok(());
        }

        Self::insufficient_permissions()
    }

    pub fn require_any_role<'a, R: Into<&'a str>>(
        &self,
        roles: Vec<R>,
    ) -> ApiResult<()> {
        if roles.into_iter().any(|r| self.require_role(r).is_ok()) {
            return Ok(());
        }

        Self::insufficient_permissions()
    }

    pub fn require_permission<A: Into<Actions>>(
        &self,
        resource: Resource,
        actions: A,
    ) -> ApiResult<()> {
        let actions: Actions = actions.into();

        if self
            .roles
            .iter()
            .any(|r| r.1.iter().any(|a| a.0 == resource && a.1.has(actions)))
        {
            return Ok(());
        }

        Self::insufficient_permissions()
    }

    pub fn require_permissions<A: Into<Actions>>(
        &self,
        permissions: Vec<(Resource, A)>,
    ) -> ApiResult<()> {
        for (resource, actions) in permissions {
            self.require_permission(resource, actions)?;
        }

        Ok(())
    }

    pub fn require_role_with_permission<
        'a,
        R: Into<&'a str>,
        A: Into<Actions>,
    >(
        &self,
        role: R,
        permission: (Resource, A),
    ) -> ApiResult<()> {
        self.require_role(role)?;
        self.require_permission(permission.0, permission.1.into())?;

        Ok(())
    }

    pub fn require_role_with_permissions<
        'a,
        R: Into<&'a str>,
        A: Into<Actions>,
    >(
        &self,
        role: R,
        permissions: Vec<(Resource, A)>,
    ) -> ApiResult<()> {
        self.require_role(role)?;
        self.require_permissions(permissions)?;

        Ok(())
    }

    pub fn require_any_role_with_permission<
        'a,
        R: Into<&'a str>,
        A: Into<Actions>,
    >(
        &self,
        roles: Vec<R>,
        permission: (Resource, A),
    ) -> ApiResult<()> {
        self.require_any_role(roles)?;
        self.require_permission(permission.0, permission.1)?;

        Ok(())
    }

    pub fn require_any_role_with_permissions<
        'a,
        R: Into<&'a str>,
        A: Into<Actions>,
    >(
        &self,
        roles: Vec<R>,
        permissions: Vec<(Resource, A)>,
    ) -> ApiResult<()> {
        self.require_any_role(roles)?;
        self.require_permissions(permissions)?;

        Ok(())
    }

    fn insufficient_permissions() -> ApiResult<()> {
        Err(ApiError::Authorization("insufficient permissions".into()))
    }
}
