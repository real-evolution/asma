use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref RE_SUPPORTED_DRIVERS: Regex = Regex::new("^(postgres)$").unwrap();
    pub static ref RE_IDENTIFIER: Regex =
        Regex::new(r#"[_a-zA-Z][_a-zA-Z0-9]{0,30}"#).unwrap();
    pub static ref RE_USERNAME: Regex =
        Regex::new(r#"^[a-zA-Z0-9](_(?!(\.|_))|\.(?!(_|\.))|[a-zA-Z0-9]){4,18}[a-zA-Z0-9]$"#).unwrap();
}
