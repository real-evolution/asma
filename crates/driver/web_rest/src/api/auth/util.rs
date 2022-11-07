use std::cmp::min;

use chrono::Utc;
use itertools::Itertools;
use kernel_entities::entities::Session;
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
            min(iat + config.timout_seconds, session.valid_until.timestamp());

        Claims {
            sub: session.id.0,
            iat,
            exp,
            iss: config.issuer.clone(),
            aud: config.audience.clone(),
            user: session.user_id.0,
            account: session.account_id.0,
            roles: Itertools::intersperse(
                access_items.iter().map(|i| i.to_string()),
                ",".to_string(),
            )
            .collect(),
        }
    }
}
