use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use local_ip_address::local_ip;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddedServerConfig {
    pub ip_address: String,
}

impl Default for EmbeddedServerConfig {
    fn default() -> Self {
        let local_ip = local_ip().unwrap_or(IpAddr::from(Ipv4Addr::UNSPECIFIED));

        Self {
            //TODO: add ports
            ip_address: local_ip.to_string(),
        }
    }
}
