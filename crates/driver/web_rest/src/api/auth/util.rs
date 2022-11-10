use std::cmp::min;

use chrono::Utc;
use itertools::Itertools;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::Session;
use kernel_services::auth::access::AppAccess;

use super::config::ApiTokenConfig;
use crate::util::jwt::Claims;

impl Claims {
    pub fn new(
        session: &Session,
        access_items: Vec<AppAccess>,
        config: &ApiTokenConfig,
    ) -> Claims {
        let iat = Utc::now().timestamp();
        let exp =
            min(iat + config.timout_seconds, session.expires_at.timestamp());

        Claims {
            sub: session.id.0,
            iat,
            exp,
            iss: config.issuer.clone(),
            aud: config.audience.clone(),
            account: session.account_id.0,
            roles: Itertools::intersperse(
                access_items.into_iter().flat_map(|i| i.into_string_vec()),
                ",".to_string(),
            )
            .collect(),
        }
    }

    pub fn to_jwt(&self, config: &ApiTokenConfig) -> anyhow::Result<String> {
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(config.signing_key.as_bytes()),
        )?;

        Ok(jwt)
    }
}
