use std::{cmp::min, collections::HashMap};

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::{Actions, Resource, Session};
use kernel_services::auth::models::AccessRule;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::ApiConfig;
use crate::error::{ApiError, ApiResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: String,
    pub account: Uuid,
    pub roles: HashMap<String, Vec<(Resource, Actions)>>,
}

impl Claims {
    pub fn new(
        session: &Session,
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
            sub: session.id.0,
            iat,
            exp,
            iss: config.token.issuer.clone(),
            aud: config.token.audience.clone(),
            account: session.account_id.0,
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
    pub fn require_role(&self, role: &str) -> ApiResult<()> {
        if self.roles.contains_key(role) {
            return Ok(());
        }

        Self::insufficient_permissions()
    }

    pub fn require_roles(&self, roles: &Vec<&str>) -> ApiResult<()> {
        for role in roles {
            self.require_role(role)?;
        }

        Ok(())
    }

    pub fn require_any_role(&self, roles: &Vec<&str>) -> ApiResult<()> {
        if roles.into_iter().any(|r| self.require_role(r).is_ok()) {
            return Ok(());
        }

        Self::insufficient_permissions()
    }

    pub fn require_permission(
        &self,
        resource: Resource,
        actions: Actions,
    ) -> ApiResult<()> {
        if self
            .roles
            .iter()
            .any(|r| r.1.iter().any(|a| a.0 == resource && a.1.has(actions)))
        {
            return Ok(());
        }

        Self::insufficient_permissions()
    }

    pub fn require_permissions(
        &self,
        permissions: Vec<(Resource, Actions)>,
    ) -> ApiResult<()> {
        for (resource, actions) in permissions {
            self.require_permission(resource, actions)?;
        }

        Ok(())
    }

    pub fn require_role_with_permission(
        &self,
        role: &str,
        permission: (Resource, Actions),
    ) -> ApiResult<()> {
        self.require_role(role)?;
        self.require_permission(permission.0, permission.1)?;

        Ok(())
    }

    pub fn require_role_with_permissions(
        &self,
        role: &str,
        permissions: Vec<(Resource, Actions)>,
    ) -> ApiResult<()> {
        self.require_role(role)?;
        self.require_permissions(permissions)?;

        Ok(())
    }

    pub fn require_any_role_with_permission(
        &self,
        roles: Vec<&str>,
        permission: (Resource, Actions),
    ) -> ApiResult<()> {
        self.require_any_role(&roles)?;
        self.require_permission(permission.0, permission.1)?;

        Ok(())
    }

    pub fn require_any_role_with_permissions(
        &self,
        roles: Vec<&str>,
        permissions: Vec<(Resource, Actions)>,
    ) -> ApiResult<()> {
        self.require_any_role(&roles)?;
        self.require_permissions(permissions)?;

        Ok(())
    }

    fn insufficient_permissions() -> ApiResult<()> {
        Err(ApiError::Authorization("insufficient permissions".into()))
    }
}
