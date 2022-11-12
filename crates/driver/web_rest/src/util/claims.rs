use std::{cmp::min, collections::HashMap};

use chrono::Utc;
use itertools::Itertools;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::Session;
use kernel_services::auth::models::AccessRule;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::ApiConfig, error::ApiResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: String,
    pub account: Uuid,
    pub roles: HashMap<String, String>,
}

impl Claims {
    pub fn new(
        session: &Session,
        access_rules: Vec<AccessRule>,
        config: &ApiConfig,
    ) -> Claims {
        let iat = Utc::now().timestamp();
        let exp = min(
            iat + config.token.timout_seconds,
            session.expires_at.timestamp(),
        );

        Claims {
            sub: session.id.0,
            iat,
            exp,
            iss: config.token.issuer.clone(),
            aud: config.token.audience.clone(),
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

    pub fn encode(&self, key: &[u8]) -> ApiResult<String> {
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(key),
        )?;

        Ok(jwt)
    }
}
