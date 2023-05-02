use std::sync::Arc;

use derive_more::{Deref, From, Into};
use driver_web_common::auth::{
    config::AUTH_TOKEN_CONFIG_SECTION, token::AuthToken,
    validator::FallbackValidator,
};
use kernel_services::config::ConfigService;
use tonic::{Request, Status};

const AUTHORIZATION_KEY: &str = "Authorization";
const BEARER_TOKEN_PREFIX: &str = "Bearer ";

#[derive(Debug, Clone, Deref, From, Into)]
pub(crate) struct GrpcAuthToken(AuthToken);

pub(crate) trait RequestExt<T> {
    fn auth<C: ConfigService>(
        &self,
        config: Arc<C>,
    ) -> Result<GrpcAuthToken, Status>;
}

impl<T> RequestExt<T> for Request<T> {
    fn auth<C: ConfigService>(
        &self,
        config: Arc<C>,
    ) -> Result<GrpcAuthToken, Status> {
        let Some(token_str) = self.metadata().get(AUTHORIZATION_KEY) else {
            return Err(Status::unauthenticated("auth header missing"));
        };

        let Ok(token_str) = token_str.to_str() else {
            return Err(Status::invalid_argument("invalid header value"));
        };

        if !token_str.starts_with(BEARER_TOKEN_PREFIX) {
            return Err(Status::invalid_argument("invalid header fromat"));
        }

        let config = match config.get_section(AUTH_TOKEN_CONFIG_SECTION) {
            | Ok(config) => config,
            | Err(err) => {
                error!("could not read auth config: {err:?}");
                return Err(Status::internal("could not read config"));
            }
        };

        match AuthToken::decode::<false, _>(
            &token_str[BEARER_TOKEN_PREFIX.len()..],
            config,
        ) {
            | Ok(token) => Ok(token.into()),
            | Err(err) => {
                error!("encode jwt token: {err:?}");
                Err(Status::internal("encoding error"))
            }
        }
    }
}

impl FallbackValidator for GrpcAuthToken {
    type Error = Status;

    fn unauthorized(&self) -> Result<&Self, Self::Error> {
        Err(Status::permission_denied("insufficient permissions"))
    }
}
