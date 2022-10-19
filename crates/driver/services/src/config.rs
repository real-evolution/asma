use kernel_services::config::{ConfigService, ConfigValue};

use config::{Config, File, FileFormat, Value, ValueKind};

use std::collections::HashMap;
use std::{env, io};

pub struct TomlConfigService {
    cfg: Config,
}

impl ConfigService for TomlConfigService {
    fn get_section<'a, C: serde::Deserialize<'a>>(
        &self,
        section: &str,
    ) -> anyhow::Result<C> {
        Ok(self.cfg.get::<C>(section)?)
    }

    fn get(&self, key: &str) -> anyhow::Result<ConfigValue> {
        let val = self.cfg.get::<Value>(key)?;

        Ok(map_config_value(val.kind))
    }

    fn get_bool(&self, key: &str) -> anyhow::Result<bool> {
        Ok(self.cfg.get_bool(key)?)
    }

    fn get_int(&self, key: &str) -> anyhow::Result<i64> {
        Ok(self.cfg.get_int(key)?)
    }

    fn get_float(&self, key: &str) -> anyhow::Result<f64> {
        Ok(self.cfg.get_float(key)?)
    }

    fn get_string(&self, key: &str) -> anyhow::Result<String> {
        Ok(self.cfg.get_string(key)?)
    }

    fn get_array(&self, key: &str) -> anyhow::Result<Vec<ConfigValue>> {
        Ok(map_config_array(self.cfg.get_array(key)?))
    }

    fn get_map(
        &self,
        key: &str,
    ) -> anyhow::Result<HashMap<String, ConfigValue>> {
        Ok(map_config_table(self.cfg.get_table(key)?))
    }
}

impl TomlConfigService {
    pub fn from_strs(strs: &[&str]) -> anyhow::Result<Self> {
        let sources: Vec<_> = strs
            .into_iter()
            .map(|s| File::from_str(*s, FileFormat::Toml))
            .collect();

        Ok(Self {
            cfg: Config::builder().add_source(sources).build()?,
        })
    }

    pub fn from_files(paths: &Vec<String>) -> anyhow::Result<Self> {
        let sources: Vec<_> = paths
            .into_iter()
            .map(|s| File::new(&s, FileFormat::Toml))
            .collect();

        Ok(Self {
            cfg: Config::builder().add_source(sources).build()?,
        })
    }

    pub fn load() -> anyhow::Result<Self> {
        let paths = Self::get_config_files()?;

        Self::from_files(&paths)
    }

    fn get_config_files() -> anyhow::Result<Vec<String>> {
        const QUALIFIER: &str = "com";
        const ORGANIZATION: &str = "SGSTel";
        const APPLICATION: &str = "asma";

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
        ValueKind::Nil => ConfigValue::None,
        ValueKind::Boolean(v) => ConfigValue::Boolean(v),
        ValueKind::I64(v) => ConfigValue::Integer(v),
        ValueKind::I128(v) => ConfigValue::Integer(v as i64),
        ValueKind::U64(v) => ConfigValue::Integer(v as i64),
        ValueKind::U128(v) => ConfigValue::Integer(v as i64),
        ValueKind::Float(v) => ConfigValue::Float(v),
        ValueKind::String(v) => ConfigValue::String(v),
        ValueKind::Table(v) => ConfigValue::Map(map_config_table(v)),
        ValueKind::Array(v) => ConfigValue::Array(map_config_array(v)),
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