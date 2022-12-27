use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref RE_IDENTIFIER: Regex =
        Regex::new(r#"^[_a-zA-Z][_a-zA-Z0-9]{0,30}$"#).unwrap();
    pub static ref RE_USERNAME: Regex =
        Regex::new(r#"^[_a-zA-Z][_a-zA-Z0-9]{2,30}$"#).unwrap();
}
