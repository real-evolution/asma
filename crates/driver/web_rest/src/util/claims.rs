use std::collections::HashSet;
use std::{cmp::min, collections::HashMap};

use chrono::Utc;
use itertools::Itertools;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
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
    pub user_display_name: String,
    pub account_id: Key<Account>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub roles: HashSet<String>,
    pub permissions: HashMap<Resource, Actions>,

    #[serde(skip)]
    pub config: ApiConfig,
}

impl Claims {
    pub fn new(
        user: User,
        account: Account,
        session: Session,
        roles: HashMap<String, Vec<(Resource, Actions)>>,
        config: ApiConfig,
    ) -> Claims {
        let iat = Utc::now().timestamp();
        let conf_exp = iat + config.token.timout_seconds;
        let exp = match session.expires_at {
            Some(session_exp) => min(conf_exp, session_exp.timestamp()),
            None => conf_exp,
        };

        let (roles, permissions): (HashSet<_>, Vec<_>) = roles.into_iter().unzip();

        let permissions = permissions
            .into_iter()
            .flatten()
            .into_grouping_map()
            .aggregate(|acc, _, val| Some(acc.unwrap_or_default() | val));

        Claims {
            sub: session.id,
            iat,
            exp,
            iss: config.token.issuer.clone(),
            aud: config.token.audience.clone(),
            user_id: user.id,
            username: user.username,
            user_display_name: user.display_name,
            account_id: account.id,
            account_name: account.account_name,
            holder_name: account.holder_name,
            roles,
            permissions,
            config,
        }
    }

    pub fn encode(&self) -> ApiResult<String> {
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(self.config.token.signing_key.as_bytes()),
        )?;

        Ok(jwt)
    }
}

impl Claims {
    #[inline]
    pub fn is_root(&self) -> ApiResult<&Self> {
        if !self.config.disable_root
            && self.roles.contains(KnownRoles::Root.into())
        {
            return Ok(self);
        }

        Self::insufficient_permissions()
    }

    #[inline]
    pub fn in_role<'a, R: Into<&'a str>>(&self, role: R) -> ApiResult<&Self> {
        self.is_root()
            .or(self.require(|| self.roles.contains(role.into())))
    }

    #[inline]
    pub fn can<A: Into<Actions> + Copy>(
        &self,
        perms: &[(Resource, A)],
    ) -> ApiResult<&Self> {
        self.is_root().or(self.require(|| {
            perms
                .iter()
                .all(|(res, act)| match self.permissions.get(res) {
                    Some(p) => p.has(act),
                    None => false,
                })
        }))
    }

    #[inline]
    pub fn is(&self, account_id: &Key<Account>) -> ApiResult<&Self> {
        self.require(|| self.account_id.value_ref() == account_id.value_ref())
    }

    #[inline]
    pub fn of(&self, user_id: &Key<User>) -> ApiResult<&Self> {
        self.require(|| self.user_id.value_ref() == user_id.value_ref())
    }

    #[inline]
    pub fn require<F: FnOnce() -> bool>(&self, req: F) -> ApiResult<&Self> {
        if req() {
            Ok(self)
        } else {
            Self::insufficient_permissions()
        }
    }

    #[inline]
    fn insufficient_permissions() -> ApiResult<&'static Self> {
        Err(ApiError::Authorization("insufficient permissions".into()))
    }
}
