#[derive(Debug)]
pub struct SqlxDataConfig<'a> {
    pub driver: &'a str,
    pub host: &'a str,
    pub port: u16,
    pub username: &'a str,
    pub password: &'a str,
}
