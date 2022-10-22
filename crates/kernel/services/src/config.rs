use std::collections::HashMap;

use anyhow::Result;
use erased_serde::Deserializer;
use serde::Deserialize;
use shaku::Interface;

#[macro_export]
macro_rules! get_config {
    ($svc:ident, $section:expr => $cfg:ty) => {
        $svc.get_section($section)?.try_into::<$cfg>()
    };
}
pub use get_config;

pub trait ConfigService: Interface {
    fn get_section<'de>(&self, section: &str) -> Result<ConfigObject<'de>>;
    fn get(&self, key: &str) -> Result<ConfigValue>;
    fn get_bool(&self, key: &str) -> Result<bool>;
    fn get_int(&self, key: &str) -> Result<i64>;
    fn get_float(&self, key: &str) -> Result<f64>;
    fn get_string(&self, key: &str) -> Result<String>;
    fn get_array(&self, key: &str) -> Result<Vec<ConfigValue>>;
    fn get_map(&self, key: &str) -> Result<HashMap<String, ConfigValue>>;
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

    pub fn try_into<D: Deserialize<'de>>(mut self) -> Result<D> {
        Ok(erased_serde::deserialize(&mut self.0)?)
    }
}
