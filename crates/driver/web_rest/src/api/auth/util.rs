use std::cmp::min;

use chrono::Utc;
use itertools::Itertools;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::Session;
use kernel_services::auth::models::AccessRule;

use super::config::ApiTokenConfig;
use crate::util::jwt::Claims;

impl Claims {
    pub fn new(
        session: &Session,
        access_rules: Vec<AccessRule>,
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
            roles: access_rules
                .into_iter()
                .map(|i| {
                    (
                        i.role_code,
                        Itertools::intersperse(
                            i.permissions.into_iter().map(|p| {
                                format!("{:X}:{:X}", p.0.repr(), p.1.inner())
                            }),
                            ",".to_string(),
                        )
                        .collect(),
                    )
                })
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
