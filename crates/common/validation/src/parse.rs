use std::net::IpAddr;

use validators::prelude::*;

use crate::patterns::*;

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
#[validator(regex(RE_IDENTIFIER))]
pub struct Identifier(pub String);

#[derive(Validator)]
#[validator(regex(RE_USERNAME))]
pub struct Username(pub String);

#[derive(Validator)]
#[validator(phone)]
pub struct PhoneNumber(pub validators::phonenumber::PhoneNumber);
