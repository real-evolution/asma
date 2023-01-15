use std::net::SocketAddr;

use anyhow::Result;
use common_validation::*;
use kernel_services::config::ConfigService;
use serde::Deserialize;
use validator::Validate;

const CONFIG_SECTION: &str = "web";

into_fn!(default_ip: String  => "127.0.0.1");
into_fn!(default_rest_port: const u16 => 3434u16);
into_fn!(default_grpc_port: const u16 => 3435u16);

#[derive(Debug, Deserialize, Validate)]
struct LanuchConfig {
    #[validate(custom = "ip_endpoint")]
    #[serde(default = "default_ip")]
    listen_address: String,

    #[validate(range(min = 0, max = 0xFFFF))]
    #[serde(default = "default_rest_port")]
    rest_listen_port: u16,

    #[validate(range(min = 0, max = 0xFFFF))]
    #[serde(default = "default_grpc_port")]
    grpc_listen_port: u16,
}

pub struct ListenAddressPair {
    pub rest: SocketAddr,
    pub grpc: SocketAddr,
}

impl ListenAddressPair {
    pub fn load<C: ConfigService>(svc: &C) -> Result<Self> {
        let conf: LanuchConfig = svc.get_section(CONFIG_SECTION)?;
        let addr = IpAddress::parse_str(conf.listen_address)?.0;

        Ok(Self {
            rest: SocketAddr::new(addr, conf.rest_listen_port),
            grpc: SocketAddr::new(addr, conf.grpc_listen_port),
        })
    }
}
