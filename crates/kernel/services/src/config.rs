use std::collections::HashMap;

use shaku::Interface;

pub trait ConfigService: Interface {
    fn get_section(
        &self,
        section: &str,
    ) -> anyhow::Result<Box<dyn erased_serde::Deserializer>>;

    fn get(&self, key: &str) -> anyhow::Result<ConfigValue>;
    fn get_bool(&self, key: &str) -> anyhow::Result<bool>;
    fn get_int(&self, key: &str) -> anyhow::Result<i64>;
    fn get_float(&self, key: &str) -> anyhow::Result<f64>;
    fn get_string(&self, key: &str) -> anyhow::Result<String>;
    fn get_array(&self, key: &str) -> anyhow::Result<Vec<ConfigValue>>;
    fn get_map(
        &self,
        key: &str,
    ) -> anyhow::Result<HashMap<String, ConfigValue>>;
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
