use std::net::SocketAddr;

use anyhow::Result;
use common_validation::*;
use kernel_services::config::ConfigService;
use serde::Deserialize;
use validator::Validate;

const CONFIG_SECTION: &str = "web";

into_fn!(default_ip: String  => "127.0.0.1");
into_fn!(default_port: const u16 => 3434u16);

#[derive(Debug, Deserialize, Validate)]
struct LanuchConfig {
    #[validate(custom = "ip_endpoint")]
    #[serde(default = "default_ip")]
    listen_address: String,

    #[validate(range(min = 0, max = 0xFFFF))]
    #[serde(default = "default_port")]
    listen_port: u16,
}

pub fn get_bind_address<'a, C: ConfigService + ?Sized>(
    svc: &'a C,
) -> Result<SocketAddr> {
    let conf: LanuchConfig = svc.get_section(CONFIG_SECTION)?;

    Ok(SocketAddr::new(
        IpAddress::parse_str(conf.listen_address)?.0,
        conf.listen_port,
    ))
}
