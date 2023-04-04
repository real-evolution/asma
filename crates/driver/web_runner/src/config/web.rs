use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

use anyhow::Result;
use serde::Deserialize;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use validator::Validate;

pub(crate) const WEB_CONFIG_SECTION: &str = "web";

into_fn!(default_rest_address: SocketAddr => SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 3434));
into_fn!(default_grpc_address: SocketAddr => SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080));

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct LanuchConfig {
    #[validate]
    pub(crate) rest: RestLanuchConfig,

    #[validate]
    pub(crate) grpc: GrpcLanuchConfig,
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub(crate) struct RestLanuchConfig {
    #[serde(default = "default_rest_address")]
    pub(crate) listen_on: SocketAddr,

    #[serde(default)]
    pub(crate) cors: CorsConfig,
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub(crate) struct GrpcLanuchConfig {
    #[serde(default = "default_grpc_address")]
    pub(crate) listen_on: SocketAddr,

    #[serde(default)]
    pub(crate) cors: CorsConfig,

    #[serde(default)]
    pub(crate) enable_http1: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
enum CorsConfigValue {
    Other(String),
    Items(Vec<String>),
}

impl CorsConfigValue {
    fn parse_as_vec<I>(&self) -> anyhow::Result<Option<Vec<I>>>
    where
        I: FromStr,
        I::Err: std::error::Error + Send + Sync + 'static,
    {
        Ok(match self {
            | CorsConfigValue::Other(ref v) => {
                if v == "*" || v.eq_ignore_ascii_case("any") {
                    None
                } else {
                    Some(vec![v.parse::<I>()?])
                }
            }
            | CorsConfigValue::Items(ref items) => {
                let items: Result<Vec<I>, _> =
                    items.iter().map(|h| h.parse::<I>()).collect();

                Some(items?)
            }
        })
    }
}

impl Default for CorsConfigValue {
    fn default() -> Self {
        Self::Items(vec![])
    }
}

#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub(crate) struct CorsConfig {
    #[serde(default)]
    allowed_methods: CorsConfigValue,

    #[serde(default)]
    allowed_headers: CorsConfigValue,

    #[serde(default)]
    allowed_origins: CorsConfigValue,
}

impl CorsConfig {
    pub(crate) fn into_layer(self) -> anyhow::Result<CorsLayer> {
        let cors = CorsLayer::new()
            .allow_headers(self.allow_headers()?)
            .allow_methods(self.allow_methods()?)
            .allow_origin(self.allow_origins()?);

        Ok(cors)
    }

    fn allow_headers(&self) -> anyhow::Result<AllowHeaders> {
        Ok(self
            .allowed_headers
            .parse_as_vec::<http::HeaderName>()?
            .map(AllowHeaders::list)
            .unwrap_or(AllowHeaders::any()))
    }

    fn allow_methods(&self) -> anyhow::Result<AllowMethods> {
        Ok(self
            .allowed_methods
            .parse_as_vec::<http::Method>()?
            .map(AllowMethods::list)
            .unwrap_or(AllowMethods::any()))
    }

    fn allow_origins(&self) -> anyhow::Result<AllowOrigin> {
        Ok(self
            .allowed_origins
            .parse_as_vec::<http::HeaderValue>()?
            .map(AllowOrigin::list)
            .unwrap_or(AllowOrigin::any()))
    }
}
