use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use chrono::Utc;
use itertools::Itertools;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use kernel_entities::{
    entities::auth::{Account, Actions, Resource, Session, User},
    traits::Key,
};
use serde::{Deserialize, Serialize};

use super::config::AuthTokenConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthToken {
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

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub holder_name: Option<String>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub roles: HashSet<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub permissions: HashMap<Resource, Actions>,

    #[serde(skip)]
    pub config: AuthTokenConfig,
}

impl AuthToken {
    pub fn new(
        user: User,
        account: Account,
        session: Session,
        roles: HashMap<String, Vec<(Resource, Actions)>>,
        config: AuthTokenConfig,
    ) -> Self {
        let iat = Utc::now().timestamp();
        let conf_exp = iat + config.timeout_seconds;
        let exp = match session.expires_at {
            | Some(session_exp) => min(conf_exp, session_exp.timestamp()),
            | None => conf_exp,
        };

        let (roles, permissions): (HashSet<_>, Vec<_>) =
            roles.into_iter().unzip();

        let permissions = permissions
            .into_iter()
            .flatten()
            .into_group_map()
            .into_iter()
            .filter_map(|(res, v)| {
                let Some(actions) = v.into_iter().fold1(|lhs, rhs| lhs | rhs) else {
                    return None;
                };

                Some((res, actions))
            })
            .collect();

        Self {
            sub: session.id,
            iat,
            exp,
            iss: config.issuer.clone(),
            aud: config.audience.clone(),
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

    pub fn encode(&self) -> anyhow::Result<String> {
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(self.config.signing_key.as_bytes()),
        )?;

        Ok(jwt)
    }

    pub fn decode<'a, J: Into<&'a str>>(
        jwt: J,
        config: AuthTokenConfig,
    ) -> anyhow::Result<Self> {
        let jwt: &str = jwt.into();

        let mut token = jsonwebtoken::decode::<Self>(
            jwt,
            &DecodingKey::from_secret(config.signing_key.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?;

        token.config = config;

        Ok(token)
    }
}
