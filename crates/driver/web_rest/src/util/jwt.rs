use std::{cmp::min, collections::HashMap};

use chrono::Utc;
use itertools::Itertools;
use jsonwebtoken::{EncodingKey, Header};
use kernel_entities::entities::auth::Session;
use kernel_services::auth::models::AccessRule;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::config::ApiTokenConfig;
use crate::error::ApiResult;

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
    pub fn encode(&self, key: &[u8]) -> ApiResult<String> {
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(key),
        )?;

        Ok(jwt)
    }
}

pub mod config {
    use common_macros::into_fn;
    use rand::distributions::{Alphanumeric, DistString};
    use serde::Deserialize;
    use validator::Validate;

    pub const API_TOKEN_CONFIG_SECTION: &str = "api.token";

    into_fn!(default_issuer: String =>  "social.sgstel.com.ye".to_string());
    into_fn!(default_audience: String => "social.sgstel.com.ye".to_string());
    into_fn!(default_timeout_seconds: const i64 =>  5 * 60);

    fn default_signing_key() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 128)
    }

    #[derive(Debug, Clone, Deserialize, Validate)]
    pub struct ApiTokenConfig {
        #[serde(default = "default_issuer")]
        pub issuer: String,

        #[serde(default = "default_issuer")]
        pub audience: String,

        #[validate(range(min = 1))]
        #[serde(default = "default_timeout_seconds")]
        pub timout_seconds: i64,

        #[serde(default = "default_signing_key")]
        pub signing_key: String,
    }
}
