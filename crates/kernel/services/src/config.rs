use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum ConfigError {
    #[display(fmt = "i/o error: {}" , _0)]
    Io(std::io::Error),

    #[display(fmt = "invalid config format")]
    InvalidConfigFormat,

    #[display(fmt = r#"section "{}" does not exist"#, _0)]
    SectionDoesNotExist(String),

    #[display(fmt = r#"key "{}" does not exist"#, _0)]
    KeyDoesNotExist(String),
}

pub trait ConfigService {
    fn get_section<'a, C: serde::Deserialize<'a>>(
        &self,
        section: &str,
    ) -> anyhow::Result<C>;

    fn get<'a>(&self, key: &str) -> anyhow::Result<&'a str>;
    fn get_as<'a, T>(&self, key: &str) -> anyhow::Result<T>;
}
