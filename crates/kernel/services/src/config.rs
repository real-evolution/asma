use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::{error::AppResult, Service};

#[async_trait::async_trait]
pub trait ConfigService: Service + Send + Sync {
    async fn reload(&self) -> AppResult<()>;

    fn get_section<T: DeserializeOwned>(
        &self,
        section: &str,
    ) -> AppResult<T>;
    fn get(&self, key: &str) -> AppResult<ConfigValue>;
    fn get_bool(&self, key: &str) -> AppResult<bool>;
    fn get_int(&self, key: &str) -> AppResult<i64>;
    fn get_float(&self, key: &str) -> AppResult<f64>;
    fn get_string(&self, key: &str) -> AppResult<String>;
    fn get_array(&self, key: &str) -> AppResult<Vec<ConfigValue>>;
    fn get_map(&self, key: &str) -> AppResult<HashMap<String, ConfigValue>>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    None,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Map(HashMap<String, ConfigValue>),
    Array(Vec<ConfigValue>),
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ConfigError {
        #[error("config key `{0}` not found")]
        NotFound(String),

        #[error("failed to parse path `{0}`")]
        PathParse(String),

        #[error("failed to parse file `{uri}`: {error}")]
        FileParse { uri: String, error: String },

        #[error("failed to parse value: {0}")]
        ValueParse(String),

        #[error("unknown error: {0}")]
        Other(String),
    }
}
