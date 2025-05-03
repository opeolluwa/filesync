use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddedServerConfig {
    pub ip_address: String,
    pub ports: Ports,
}

impl Default for EmbeddedServerConfig {
    fn default() -> Self {
        let local_ip = local_ip().unwrap_or(IpAddr::from(Ipv4Addr::UNSPECIFIED));

        Self {
            //TODO: add ports
            ip_address: local_ip.to_string(),
            ports: Ports::default(),
        }
    }
}

pub const HTTP_PORT: u64 = 18005;
pub const HTTPS_PORT: u64 = 18006;
pub const KEY_CHAIN_PATH: &str = "certificates";
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Ports {
    pub http: u16,
    pub https: u16,
}

impl Default for Ports {
    fn default() -> Self {
        Self {
            http: HTTP_PORT as u16,
            https: HTTPS_PORT as u16,
        }
    }
}
