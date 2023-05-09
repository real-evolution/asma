use std::{
    collections::HashMap,
    env, io,
    sync::{Arc, Mutex, MutexGuard},
};

use config::{Config, File, FileFormat, Value, ValueKind};
use derive_more::Constructor;
use kernel_services::{
    config::*,
    error::{AppResult, ConfigError},
    Service,
};
use serde::de::DeserializeOwned;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "SGSTel";
const APPLICATION: &str = "asma";
const CONFIG_ENV_KEY: &str = "ASMA_CONFIG";

#[derive(Constructor, Default)]
pub struct TomlConfigService(Arc<Mutex<Config>>);

#[async_trait::async_trait]
impl ConfigService for TomlConfigService {
    async fn reload(&self) -> AppResult<()> {
        debug!(
            "loading config from toml files: {:?}",
            TomlConfigService::get_config_files()?
        );

        let sources: Vec<_> = Self::get_config_files()?
            .into_iter()
            .map(|s| File::new(&s, FileFormat::Toml))
            .collect();

        *self.inner()? = Config::builder()
            .add_source(sources)
            .build()
            .map_err(anyhow::Error::new)?;

        Ok(())
    }

    fn get_section<'de, T: DeserializeOwned>(
        &self,
        section: &str,
    ) -> AppResult<T> {
        debug!("reading configuration section `{section}`");

        let val = self
            .inner()?
            .get::<Value>(section)
            .map_err(map_config_error)?
            .try_deserialize::<T>()
            .map_err(map_config_error)?;

        Ok(val)
    }

    fn get(&self, key: &str) -> AppResult<ConfigValue> {
        debug!("reading a configuration object with key `{key}`");

        let val = self.inner()?.get::<Value>(key).map_err(map_config_error)?;

        Ok(map_config_value(val.kind))
    }

    fn get_bool(&self, key: &str) -> AppResult<bool> {
        debug!("reading a configuration boolean with key `{key}`");

        Ok(self.inner()?.get_bool(key).map_err(map_config_error)?)
    }

    fn get_int(&self, key: &str) -> AppResult<i64> {
        debug!("reading a configuration integer with key `{key}`");

        Ok(self.inner()?.get_int(key).map_err(map_config_error)?)
    }

    fn get_float(&self, key: &str) -> AppResult<f64> {
        debug!("reading a configuration float with key `{key}`");

        Ok(self.inner()?.get_float(key).map_err(map_config_error)?)
    }

    fn get_string(&self, key: &str) -> AppResult<String> {
        debug!("reading a configuration string with key `{key}`");

        Ok(self.inner()?.get_string(key).map_err(map_config_error)?)
    }

    fn get_array(&self, key: &str) -> AppResult<Vec<ConfigValue>> {
        debug!("reading a configuration array with key `{key}`");

        Ok(map_config_array(
            self.inner()?.get_array(key).map_err(map_config_error)?,
        ))
    }

    fn get_map(&self, key: &str) -> AppResult<HashMap<String, ConfigValue>> {
        debug!("reading a configuration map with key `{key}`");

        Ok(map_config_table(
            self.inner()?.get_table(key).map_err(map_config_error)?,
        ))
    }
}

#[async_trait::async_trait]
impl Service for TomlConfigService {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        self.reload().await
    }
}

impl TomlConfigService {
    fn inner(&self) -> AppResult<MutexGuard<'_, Config>> {
        let lck = self
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;

        Ok(lck)
    }

    fn get_config_files() -> anyhow::Result<Vec<String>> {
        if let Ok(path) = env::var(CONFIG_ENV_KEY) {
            return Ok(vec![path]);
        }

        let mut config_dir = directories::ProjectDirs::from(
            QUALIFIER,
            ORGANIZATION,
            APPLICATION,
        )
        .unwrap()
        .config_dir()
        .to_path_buf()
        .canonicalize()?;

        if !config_dir.exists() {
            anyhow::bail!(io::Error::new(
                io::ErrorKind::NotFound,
                config_dir.to_str().unwrap()
            ));
        }

        config_dir.push("asma.toml");
        let mut config_files = vec![config_dir.to_str().unwrap().into()];

        if let Ok(mode) = env::var("RUN_MODE") {
            config_dir.pop();
            config_dir.push(format!("asma_{}.toml", mode.to_lowercase()));
            config_files.push(config_dir.to_str().unwrap().into());
        }

        Ok(config_files)
    }
}

fn map_config_value(val: ValueKind) -> ConfigValue {
    match val {
        | ValueKind::Nil => ConfigValue::None,
        | ValueKind::Boolean(v) => ConfigValue::Boolean(v),
        | ValueKind::I64(v) => ConfigValue::Integer(v),
        | ValueKind::I128(v) => ConfigValue::Integer(v as i64),
        | ValueKind::U64(v) => ConfigValue::Integer(v as i64),
        | ValueKind::U128(v) => ConfigValue::Integer(v as i64),
        | ValueKind::Float(v) => ConfigValue::Float(v),
        | ValueKind::String(v) => ConfigValue::String(v),
        | ValueKind::Table(v) => ConfigValue::Map(map_config_table(v)),
        | ValueKind::Array(v) => ConfigValue::Array(map_config_array(v)),
    }
}

fn map_config_table(
    val: HashMap<String, Value>,
) -> HashMap<String, ConfigValue> {
    val.into_iter()
        .map(|(k, v)| (k, map_config_value(v.kind)))
        .collect()
}

fn map_config_array(val: Vec<Value>) -> Vec<ConfigValue> {
    val.into_iter().map(|v| map_config_value(v.kind)).collect()
}

fn map_config_error(err: config::ConfigError) -> ConfigError {
    match err {
        | config::ConfigError::NotFound(key) => ConfigError::NotFound(key),
        | config::ConfigError::Message(msg) => ConfigError::Other(msg),

        | config::ConfigError::Frozen => {
            ConfigError::Other("config is frozen".into())
        }

        | config::ConfigError::PathParse(kind) => {
            ConfigError::PathParse(kind.description().into())
        }

        | config::ConfigError::FileParse { uri, cause } => {
            ConfigError::FileParse {
                uri: uri.unwrap_or_else(|| "<unknown>".into()),
                error: cause.to_string(),
            }
        }

        | config::ConfigError::Type {
            origin,
            unexpected,
            expected,
            key,
        } => ConfigError::ValueParse(format!(
            "key: {key:?}, origin: {origin:?}, expected: {expected}, got: \
             {unexpected}"
        )),

        | config::ConfigError::Foreign(err) => {
            ConfigError::Other(err.to_string())
        }
    }
}
