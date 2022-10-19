use crate::patterns::*;

use validators::prelude::*;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(NotAllow)))]
pub struct Host(pub String);

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Allow)))]
pub struct Endpoint {
    pub domain: String,
    pub port: Option<u16>,
}

#[derive(Validator)]
#[validator(regex(RE_SUPPORTED_DRIVERS))]
pub struct SupportedDriver(pub String);

#[derive(Validator)]
#[validator(regex(RE_BASIC_USERNAME))]
pub struct BasicUsername(pub String);

#[derive(Validator)]
#[validator(regex(RE_USERNAME))]
pub struct Username(pub String);
