use std::{cmp::min, collections::HashMap};

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
use kernel_services::auth::models::AccessRule;
use serde::{Deserialize, Serialize};

use super::claims_macros::ClaimsRequirement;
use crate::config::ApiConfig;
use crate::error::ApiResult;

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
    pub roles: HashMap<String, Vec<(Resource, Actions)>>,

    #[serde(skip)]
    pub config: ApiConfig,
}

impl Claims {
    pub fn new(
        user: User,
        account: Account,
        session: Session,
        access_rules: Vec<AccessRule>,
        config: ApiConfig,
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
            user_display_name: user.display_name,
            account_id: account.id,
            account_name: account.account_name,
            holder_name: account.holder_name,
            roles: access_rules
                .into_iter()
                .map(|a| (a.role_code, a.permissions))
                .collect(),
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
    pub fn check<'a>(&'a self) -> ClaimsRequirement<'a> {
        ClaimsRequirement::new(self)
    }
}
