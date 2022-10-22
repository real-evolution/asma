use crate::patterns::*;

use validators::prelude::*;

use std::net::IpAddr;

#[derive(Validator)]
#[validator(host(local(Allow), port(Allow)))]
pub struct Host {
    pub host: validators::models::Host,
    pub port: Option<u16>,
}

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Allow)))]
pub struct Endpoint {
    pub domain: String,
    pub port: Option<u16>,
}

#[derive(Validator)]
#[validator(ip(local(Allow), port(Allow)))]
pub struct IpEndpoint {
    pub ip: IpAddr,
    pub port: Option<u16>,
}

#[derive(Validator)]
#[validator(ip(local(Allow), port(NotAllow)))]
pub struct IpAddress(pub IpAddr);

#[derive(Validator)]
#[validator(regex(RE_SUPPORTED_DRIVERS))]
pub struct SupportedDriver(pub String);

#[derive(Validator)]
#[validator(regex(RE_IDENTIFIER))]
pub struct Identifier(pub String);

#[derive(Validator)]
#[validator(regex(RE_USERNAME))]
pub struct Username(pub String);
