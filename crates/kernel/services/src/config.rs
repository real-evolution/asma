use std::collections::HashMap;

use erased_serde::Deserializer;
use serde::Deserialize;

#[macro_export]
macro_rules! get_config {
    ($svc:expr, $section:expr => $cfg:ty) => {
        $svc.get_section($section)?.try_into::<$cfg>()
    };
}
pub use get_config;

use crate::error::AppResult;

pub trait ConfigService: Send + Sync {
    fn get_section<'de>(&self, section: &str) -> AppResult<ConfigObject<'de>>;
    fn get(&self, key: &str) -> AppResult<ConfigValue>;
    fn get_bool(&self, key: &str) -> AppResult<bool>;
    fn get_int(&self, key: &str) -> AppResult<i64>;
    fn get_float(&self, key: &str) -> AppResult<f64>;
    fn get_string(&self, key: &str) -> AppResult<String>;
    fn get_array(&self, key: &str) -> AppResult<Vec<ConfigValue>>;
    fn get_map(&self, key: &str) -> AppResult<HashMap<String, ConfigValue>>;
}

pub struct ConfigObject<'de>(Box<dyn Deserializer<'de>>);

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

impl<'de> ConfigObject<'de> {
    pub fn new(value: Box<dyn Deserializer<'de>>) -> Self {
        Self(value)
    }

    pub fn try_into<D: Deserialize<'de>>(mut self) -> AppResult<D> {
        Ok(erased_serde::deserialize(&mut self.0)?)
    }
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

        #[error("deserialization error: {0}")]
        Deserialization(#[from] erased_serde::Error),

        #[error("unknown error: {0}")]
        Other(String),
    }

    impl From<erased_serde::Error> for crate::error::AppError {
        fn from(err: erased_serde::Error) -> Self {
            Into::<ConfigError>::into(err).into()
        }
    }
}
