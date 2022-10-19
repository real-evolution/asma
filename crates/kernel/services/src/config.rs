use std::collections::HashMap;

pub trait ConfigService {
    fn get_section_validated<'a, C>(&self, section: &str) -> anyhow::Result<C>
    where
        C: serde::Deserialize<'a> + validator::Validate;

    fn get_section<'a, C: serde::Deserialize<'a>>(
        &self,
        section: &str,
    ) -> anyhow::Result<C>;

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
