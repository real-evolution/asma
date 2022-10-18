
pub trait ConfigService {
    fn get_section<'a, C: serde::Deserialize<'a>>(
        &self,
        section: &str,
    ) -> anyhow::Result<C>;

    fn get<'a>(&self, key: &str) -> anyhow::Result<&'a str>;
    fn get_as<'a, T>(&self, key: &str) -> anyhow::Result<T>;
}
